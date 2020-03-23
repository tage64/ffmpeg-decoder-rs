// TODO do w/out the unions?
//#![feature(untagged_unions)]
#![allow(improper_ctypes)]

pub mod avcodec;
pub mod avfilter;
pub mod avformat;
pub mod avresample;
pub mod avutil;
pub mod swresample;

#[cfg(test)]
mod tests {
    use super::avcodec::avcodec_configuration;
    use super::avformat::avformat_configuration;
    use super::avresample::avresample_configuration;
    use std::ffi::CStr;
    #[test]
    fn version() {
        unsafe {
            println!(
                "{}{}{}",
                CStr::from_ptr(avcodec_configuration()).to_string_lossy(),
                CStr::from_ptr(avformat_configuration()).to_string_lossy(),
                CStr::from_ptr(avresample_configuration()).to_string_lossy()
            )
        };
    }
}