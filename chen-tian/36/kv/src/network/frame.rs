use std::io::Write;

use bytes::{BufMut, BytesMut};
use flate2::{write::GzEncoder, Compression};
use prost::Message;
use tracing::debug;

use crate::KvError;


/// 长度整个占用 4 个字节
pub const LEN_LEN: usize = 4;

/// 长度占 31 bit，所以最大的 frame 是 2G
const MAX_FRAME: usize = 2 * 1024 * 1024 * 1024;

/// 如果 payload 超过了 1436 字节，就做压缩 
const COMPRESSION_LIMIT: usize = 1436;

/// 代表压缩的 bit （整个长度 4 字节的最高位）
const COMPRESSION_BIT: usize = 1 << 31;

/// 处理 Frame 的 encode/decode
pub trait FrameCoder
where
    Self: Message + Sized + Default,
{
    /// 把一个 Message encode 成一个 frame
    fn encode_from(&self, buf: &mut BytesMut) -> Result<(), KvError> {
        let size = self.encoded_len();

        if size > MAX_FRAME {
            return Err(KvError::FrameError);
        }

        // 我们先写入长度，如果需要压缩，再重写压缩后的长度
        buf.put_u32(size as _);

        if size > COMPRESSION_LIMIT {
            let mut buf1 = Vec::with_capacity(size);
            self.encode(&mut buf1);

            // BytesMut 支持逻辑上的 split （之后还能 unsplit)
            // 所以我们先把长度这 4 字节拿走，清除
            let payload = buf.split_off(LEN_LEN);
            buf.clear();

            // 处理 gzip 压缩，具体可以参考 flate2 文档
            let mut encoder = GzEncoder::new(payload.writer(),
             Compression::default());
            encoder.write_all(&buf1[..])?;

            let payload = encoder.finish()?.into_inner();
            debug!("Encode a frame: size {}{}", size, payload.len());

            // 写入压缩后的长度
            buf.put_u32((payload.len() | COMPRESSION_BIT) as _);

            // 把 BytesMut 再合并回来
            buf.unsplit(payload);

            Ok(())
        } else {
            self.encode(buf)?;
            Ok(())
        }


    }
}