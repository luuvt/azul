/// Callback for rendering iframes (infinite data structures that have to know how large they are rendered)
pub type AzIFrameCallbackType = fn(AzIFrameCallbackInfoPtr) -> AzIFrameCallbackReturn;