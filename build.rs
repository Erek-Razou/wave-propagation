fn main() {
    cc::Build::new()
        .cpp(true)
        .file("LFMF/src/Airy.cpp")
        .file("LFMF/src/FlatEarthCurveCorrection.cpp")
        .file("LFMF/src/LFMF.cpp")
        .file("LFMF/src/ResidueSeries.cpp")
        .file("LFMF/src/ValidateInputs.cpp")
        .file("LFMF/src/WiRoot.cpp")
        .file("LFMF/src/wofz.cpp")
        .compile("lfmf");
}
