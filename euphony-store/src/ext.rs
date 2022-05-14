use euphony_compiler::Hash;
use euphony_units::coordinates::Cartesian;
use std::io;

pub trait ReadExt {
    fn read_coordinate(&mut self) -> io::Result<Cartesian<f64>>;
    fn read_u64(&mut self) -> io::Result<u64>;
    fn read_f32(&mut self) -> io::Result<f32>;
    fn read_f64(&mut self) -> io::Result<f64>;
    fn read_hash(&mut self) -> io::Result<Hash>;
}

impl<R: io::Read> ReadExt for R {
    #[inline]
    fn read_coordinate(&mut self) -> io::Result<Cartesian<f64>> {
        let x = self.read_f32()? as f64;
        let y = self.read_f32()? as f64;
        let z = self.read_f32()? as f64;
        let value = Cartesian { x, y, z };
        Ok(value)
    }

    #[inline]
    fn read_u64(&mut self) -> io::Result<u64> {
        let mut value = [0u8; 8];
        self.read_exact(&mut value)?;
        let value = u64::from_ne_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_f32(&mut self) -> io::Result<f32> {
        let mut value = [0u8; 4];
        self.read_exact(&mut value)?;
        let value = f32::from_ne_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_f64(&mut self) -> io::Result<f64> {
        let mut value = [0u8; 8];
        self.read_exact(&mut value)?;
        let value = f64::from_ne_bytes(value);
        Ok(value)
    }

    #[inline]
    fn read_hash(&mut self) -> io::Result<Hash> {
        let mut value = Hash::default();
        self.read_exact(&mut value)?;
        Ok(value)
    }
}
