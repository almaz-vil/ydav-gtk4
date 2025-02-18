fn main(){
    glib_build_tools::compile_resources(
        &["resource"],
        "resource/resources.gresource.xml",
        "compiled.gresource",
    );

}