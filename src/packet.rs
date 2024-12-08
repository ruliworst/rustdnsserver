pub struct BytePacketBuffer {
  buf: [u8; 512],
  pos: usize,
}

impl BytePacketBuffer {
  pub fn new() -> BytePacketBuffer {
    BytePacketBuffer { buf: [0; 512], pos: 0 }
  }

  pub fn pos(&self) -> usize {
    self.pos
  }

  pub fn step(&mut self, steps: usize) -> Result<(), String> {
    if self.pos + steps >= self.buf.len() {
      return Err(self.exceeded_buffer_capacity(steps));
    }

    self.pos += steps;

    Ok(())
  }

  fn exceeded_buffer_capacity(&self, steps: usize) -> String {
    format!(
      "The number of steps {} exceeded from position {} the maximum capacity of the buffer ({}).",
      steps,
      self.pos,
      self.buf.len(),
    )
  }

  pub fn seek(&mut self, pos: usize) -> Result<(), String> {
    if pos >= self.buf.len() {
      return Err(self.exceeded_maximum_seek(pos));
    }

    self.pos = pos;

    Ok(())
  }

  fn exceeded_maximum_seek(&self, pos: usize) -> String {
    format!(
      "The position {} is not valid because the maximum position is {}.",
      pos,
      self.buf.len() - 1,
    )
  }

  pub fn get(&self, pos: usize) -> Result<u8, String> {
    if self.pos >= self.buf.len() {
      return Err(Self::reached_maximum_position());
    }

    Ok(self.buf[pos])
  }

  pub fn read(&mut self) -> Result<u8, String> {
    let result: u8 = self.get(self.pos)?;
    self.pos += 1;

    Ok(result)
  }

  fn reached_maximum_position() -> String {
    format!("Maximum position reached so, no more bytes can be read.")
  }

  pub fn get_range(&self, start: usize, len: usize) -> Result<&[u8], String> {
    if start + len >= self.buf.len() {
      return Err(self.exceeded_range(start, len));
    }

    Ok(&self.buf[start..start + len as usize])
  }

  fn exceeded_range(&self, start: usize, len: usize) -> String {
    format!(
      "The range starting from {} with length {} exceeded the limit ({}).",
      start,
      len,
      self.buf.len()
    )
  }

  pub fn read_u16(&mut self) -> Result<u16, String> {
    let res: u16 = ((self.read()? as u16) << 8) | (self.read()? as u16);

    Ok(res)
  }

  pub fn read_u32(&mut self) -> Result<u32, String> {
    let res: u32 = ((self.read()? as u32) << 24)
      | ((self.read()? as u32) << 16)
      | ((self.read()? as u32) << 8)
      | ((self.read()? as u32) << 0);

    Ok(res)
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_buffer_initialization() {
    let buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert_eq!(buffer.pos, 0);
    assert_eq!(buffer.buf.len(), 512);
  }

  #[test]
  fn test_initial_position() {
    let buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert_eq!(buffer.pos(), 0);
  }

  #[test]
  fn test_step_within_limits() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert!(buffer.step(10).is_ok());
    assert_eq!(buffer.pos(), 10);
  }

  #[test]
  fn test_seek_within_limits() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert!(buffer.seek(152).is_ok());
    assert_eq!(buffer.pos(), 152);
  }

  #[test]
  fn test_step_beyond_limits() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    let result: Result<(), String> = buffer.step(600);
    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "The number of steps 600 exceeded from position 0 the maximum capacity of the buffer (512)."
    );
  }

  #[test]
  fn test_seek_beyond_limits() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    let result: Result<(), String> = buffer.seek(512);
    assert!(result.is_err());
    assert_eq!(
      result.unwrap_err(),
      "The position 512 is not valid because the maximum position is 511."
    );
  }

  #[test]
  fn test_step_to_the_boundary() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert!(buffer.step(511).is_ok());
    assert_eq!(buffer.pos(), 511);
  }

  #[test]
  fn test_step_exact_overflow() {
    let mut buffer: BytePacketBuffer = BytePacketBuffer::new();
    assert!(buffer.step(512).is_err());
  }
}
