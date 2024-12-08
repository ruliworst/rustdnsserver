mod packet;

use packet::BytePacketBuffer;

fn main() {
  // Create a new buffer and use all methods
  let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
  println!("Initial position: {}", buffer.pos());
  buffer.step(10).unwrap();
  println!("Position after step: {}", buffer.pos());
  buffer.seek(152).unwrap();
  println!("Position after seek: {}", buffer.pos());
  println!("Get byte at position 256: {}", buffer.get(256).unwrap());
  buffer.seek(140).unwrap();
  println!("Read byte at position 140: {}", buffer.read().unwrap());
  println!("Buffer position: {}", buffer.pos());
  buffer.seek(0).unwrap();
  println!("Buffer range from 0 to 9: {:?}", buffer.get_range(0, 10).unwrap());
  println!("Buffer read u16: {:?}", buffer.read_u16().unwrap());
  println!("Buffer read u32: {:?}", buffer.read_u32().unwrap());
}
