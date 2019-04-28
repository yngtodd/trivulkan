use vulkano::instance::Instance;
use vulkano::instance::InstanceExtensions;
use vulkano::instance::PhysicalDevice;

use vulkano::device::Device;
use vulkano::device::DeviceExtensions;
use vulkano::device::Features;

use vulkano::buffer::BufferUsage;
use vulkano::buffer::CpuAccessibleBuffer;
use vulkano::command_buffer::CommandBuffer;
use vulkano::command_buffer::AutoCommandBufferBuilder;

use vulkano::sync::GpuFuture;

fn main() {
    let instance = Instance::new(None, &InstanceExtensions::none(), None)
        .expect("Failed to create an instance.");

    let physical = PhysicalDevice::enumerate(&instance).next().expect("No device available.");

    for family in physical.queue_families() {
        println!("Found a queue family with {:?} queue(s)", family.queues_count());
    }

    let queue_family = physical.queue_families()
        .find(|&q| q.supports_graphics())
        .expect("Couldn't find a graphical queue family.");

    let (device, mut queues) = {
        Device::new(physical, &Features::none(), &DeviceExtensions::none(),
                   [(queue_family, 0.5)].iter().cloned()).expect("Failed to create device.")
    };

    let queue = queues.next().unwrap();
   
    let source_content = 0..64;
    let source = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(), 
                                                source_content).expect("Failed to create source buffer.");

    let dest_content = (0..64).map(|_| 0);
    let dest = CpuAccessibleBuffer::from_iter(device.clone(), BufferUsage::all(),
                                              dest_content).expect("Failed to create destination buffer");

    let command_buffer = AutoCommandBufferBuilder::new(device.clone(), queue.family()).unwrap()
        .copy_buffer(source.clone(), dest.clone()).unwrap()
        .build().unwrap();

    let finished = command_buffer.execute(queue.clone()).unwrap();
    finished.then_signal_fence_and_flush().unwrap()
        .wait(None).unwrap();

    let src_content = source.read().unwrap();
    let dest_content = dest.read().unwrap();

    assert_eq!(&*src_content, &*dest_content);
}
