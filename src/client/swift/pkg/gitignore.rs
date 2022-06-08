use crate::core::graph::Graph;


pub(crate) async fn generate_gitignore(_graph: &'static Graph) -> String {
    format!(r#".DS_Store
/.build
/Packages
/*.xcodeproj
xcuserdata/
DerivedData/
.swiftpm/xcode/package.xcworkspace/contents.xcworkspacedata
"#)
}
