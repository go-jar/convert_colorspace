use crate::vpx::*;
use std::os::raw::c_int;

const STRIDE_ALIGN: usize = 16; // commonly used in libvpx vpx_img_alloc caller

extern "C" {
    // seems libyuv uses reverse byte order compared with our view

    pub fn ARGBRotate(
        src_argb: *const u8,
        src_stride_argb: c_int,
        dst_argb: *mut u8,
        dst_stride_argb: c_int,
        src_width: c_int,
        src_height: c_int,
        mode: c_int,
    ) -> c_int;

    pub fn ARGBMirror(
        src_argb: *const u8,
        src_stride_argb: c_int,
        dst_argb: *mut u8,
        dst_stride_argb: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // bgra to yuv
    pub fn ARGBToI420(
        src_bgra: *const u8,
        src_stride_bgra: c_int,
        dst_y: *mut u8,
        dst_stride_y: c_int,
        dst_u: *mut u8,
        dst_stride_u: c_int,
        dst_v: *mut u8,
        dst_stride_v: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // rgba to yuv
    // ABGR little endian (rgba in memory) to I420.
    pub fn ABGRToI420(
        src_abgr: *const u8,
        src_stride_abgr: ::std::os::raw::c_int,
        dst_y: *mut u8,
        dst_stride_y: ::std::os::raw::c_int,
        dst_u: *mut u8,
        dst_stride_u: ::std::os::raw::c_int,
        dst_v: *mut u8,
        dst_stride_v: ::std::os::raw::c_int,
        width: ::std::os::raw::c_int,
        height: ::std::os::raw::c_int,
    ) -> ::std::os::raw::c_int;

    pub fn NV12ToI420(
        src_y: *const u8,
        src_stride_y: c_int,
        src_uv: *const u8,
        src_stride_uv: c_int,
        dst_y: *mut u8,
        dst_stride_y: c_int,
        dst_u: *mut u8,
        dst_stride_u: c_int,
        dst_v: *mut u8,
        dst_stride_v: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // yuv to rgb
    pub fn I420ToRAW(
        src_y: *const u8,
        src_stride_y: c_int,
        src_u: *const u8,
        src_stride_u: c_int,
        src_v: *const u8,
        src_stride_v: c_int,
        dst_rgba: *mut u8,
        dst_stride_raw: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // yub to bgra
    pub fn I420ToARGB(
        src_y: *const u8,
        src_stride_y: c_int,
        src_u: *const u8,
        src_stride_u: c_int,
        src_v: *const u8,
        src_stride_v: c_int,
        dst_rgba: *mut u8,
        dst_stride_rgba: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // rgb to yuv
    // RGB big endian (rgb in memory) to I420.
    pub fn RAWToI420(
        src_raw: *const u8,
        src_stride_raw: c_int,
        dst_y: *mut u8,
        dst_stride_y: c_int,
        dst_u: *mut u8,
        dst_stride_u: c_int,
        dst_v: *mut u8,
        dst_stride_v: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;

    // RGB big endian (rgb in memory) to RGBA.
    pub fn RAWToRGBA(
        src_raw: *const u8,
        src_stride_raw: c_int,
        dst_rgba: *mut u8,
        dst_stride_rgba: c_int,
        width: c_int,
        height: c_int,
    ) -> c_int;
}

// https://github.com/webmproject/libvpx/blob/master/vpx/src/vpx_image.c
#[inline]
fn get_vpx_i420_stride(
    width: usize,
    height: usize,
    stride_align: usize,
) -> (usize, usize, usize, usize, usize, usize) {
    let mut img = Default::default();
    unsafe {
        vpx_img_wrap(
            &mut img,
            vpx_img_fmt::VPX_IMG_FMT_I420,
            width as _,
            height as _,
            stride_align as _,
            0x1 as _,
        );
    }
    (
        img.w as _,
        img.h as _,
        img.stride[0] as _,
        img.stride[1] as _,
        img.planes[1] as usize - img.planes[0] as usize,
        img.planes[2] as usize - img.planes[0] as usize,
    )
}

pub fn rgb_to_i420(width: usize, height: usize, src: &[u8], dst: &mut Vec<u8>) {
    let (_, _, dst_stride_y, dst_stride_uv, u, v) =
        get_vpx_i420_stride(width, height, STRIDE_ALIGN);
    dst.resize(width * height * 3 / 2, 0);
    let dst_y = dst.as_mut_ptr();
    let dst_u = dst[u..].as_mut_ptr();
    let dst_v = dst[v..].as_mut_ptr();

    unsafe {
        RAWToI420(
            src.as_ptr(),
            (src.len() / height) as _,
            dst_y,
            dst_stride_y as _,
            dst_u,
            dst_stride_uv as _,
            dst_v,
            dst_stride_uv as _,
            width as _,
            height as _,
        );
    };
}

pub fn i420_to_rgb(width: usize, height: usize, src: &[u8], dst: &mut Vec<u8>) {
    let (_, _, src_stride_y, src_stride_uv, u, v) =
        get_vpx_i420_stride(width, height, STRIDE_ALIGN);
    let src_y = src.as_ptr();
    let src_u = src[u..].as_ptr();
    let src_v = src[v..].as_ptr();
    dst.resize(width * height * 3, 0);
    unsafe {
        I420ToRAW(
            src_y,
            src_stride_y as _,
            src_u,
            src_stride_uv as _,
            src_v,
            src_stride_uv as _,
            dst.as_mut_ptr(),
            (width * 3) as _,
            width as _,
            height as _,
        );
    };
}

pub fn rgba_to_i420(width: usize, height: usize, src: &[u8], dst: &mut Vec<u8>) {
    let (_, h, dst_stride_y, dst_stride_uv, u, v) =
        get_vpx_i420_stride(width, height, STRIDE_ALIGN);
    let bps = 12;
    dst.resize(h * dst_stride_y * bps / 8, 0);
    let dst_y = dst.as_mut_ptr();
    let dst_u = dst[u..].as_mut_ptr();
    let dst_v = dst[v..].as_mut_ptr();
    unsafe {
        ABGRToI420(
            src.as_ptr(),
            (src.len() / height) as _,
            dst_y,
            dst_stride_y as _,
            dst_u,
            dst_stride_uv as _,
            dst_v,
            dst_stride_uv as _,
            width as _,
            height as _,
        );
    };
}

pub fn bgra_to_i420(width: usize, height: usize, src: &[u8], dst: &mut Vec<u8>) {
    let (_, h, dst_stride_y, dst_stride_uv, u, v) =
        get_vpx_i420_stride(width, height, STRIDE_ALIGN);
    let bps = 12;
    dst.resize(h * dst_stride_y * bps / 8, 0);
    let dst_y = dst.as_mut_ptr();
    let dst_u = dst[u..].as_mut_ptr();
    let dst_v = dst[v..].as_mut_ptr();
    unsafe {
        ARGBToI420(
            src.as_ptr(),
            (src.len() / height) as _,
            dst_y,
            dst_stride_y as _,
            dst_u,
            dst_stride_uv as _,
            dst_v,
            dst_stride_uv as _,
            width as _,
            height as _,
        );
    }
}

pub unsafe fn nv12_to_i420(
    src_y: *const u8,
    src_stride_y: c_int,
    src_uv: *const u8,
    src_stride_uv: c_int,
    width: usize,
    height: usize,
    dst: &mut Vec<u8>,
) {
    let (w, h, dst_stride_y, dst_stride_uv, u, v) =
        get_vpx_i420_stride(width, height, STRIDE_ALIGN);
    let bps = 12;
    dst.resize(h * w * bps / 8, 0);
    let dst_y = dst.as_mut_ptr();
    let dst_u = dst[u..].as_mut_ptr();
    let dst_v = dst[v..].as_mut_ptr();
    NV12ToI420(
        src_y,
        src_stride_y,
        src_uv,
        src_stride_uv,
        dst_y,
        dst_stride_y as _,
        dst_u,
        dst_stride_uv as _,
        dst_v,
        dst_stride_uv as _,
        width as _,
        height as _,
    );
}
