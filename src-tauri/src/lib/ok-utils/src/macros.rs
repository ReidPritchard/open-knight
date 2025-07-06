/// Wrapper macro for ts-rs that uses a consistent export path
/// Mimics the behavior of a constant path which rs-ts does not support
///
#[macro_export]
macro_rules! ts_export {
    ($struct:item) => {
        #[derive(Debug, Clone, Serialize, Deserialize, TS)]
        #[ts(export, export_to = "../../src/shared/bindings/")]
        $struct
    };
}
