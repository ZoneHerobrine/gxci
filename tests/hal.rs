#[cfg(test)]
mod tests {
    use super::*;
    use gxci::*;
    use gxci::raw::gx_enum::GX_STATUS_LIST;

    #[test]
    fn test_gxci_init_and_close() {
        let dll_path = "C:\\Program Files\\Daheng Imaging\\GalaxySDK\\APIDll\\Win64\\GxIAPI.dll"; // 假设这是一个测试用的 DLL 路径

        // 初始化 GXInstance
        gxci_init(dll_path);

        // 验证 GX_INSTANCE 是否被正确设置
        unsafe {
            assert!(GXI.is_some());
        }

        // 关闭 GXInstance
        let status = gxci_close();
        // 这里根据实际情况验证关闭函数返回的状态
        assert_eq!(status, GX_STATUS_LIST::GX_STATUS_SUCCESS);

        // 再次初始化，确保可以重新初始化
        gxci_init(dll_path);
        // 验证重新初始化后的状态
        unsafe {
            assert!(GXI.is_some());
        }

        // 最后关闭
        let status = gxci_close();
        assert_eq!(status, GX_STATUS_LIST::GX_STATUS_SUCCESS);
    }
}
