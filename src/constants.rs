// exports.testGeneratedCode =
// " ONE.foo=function(a){return baz(a);};\n TWO.inc=function(a){return a+1;};";
pub(crate) const testGeneratedCode: &str =
    r##" ONE.foo=function(a){return baz(a);};\n TWO.inc=function(a){return a+1;};"##;
// exports.testMap = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz", "n"],
// sources: ["one.js", "two.js"],
// sourceRoot: "/the/root",
// mappings:
// "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
// };
pub(crate) const testMap: &str = r##"{
  "version": 3,
  "file": "min.js",
  "names": ["bar", "baz", "n"],
  "sources": ["one.js", "two.js"],
  "sourceRoot": "/the/root",
  "mappings":
  "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}"##;

// exports.testMapNoSourceRoot = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz", "n"],
// sources: ["one.js", "two.js"],
// mappings:
// "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
// };
pub(crate) const testMapNoSourceRoot: &str = r##" {
 version: 3,
 file: "min.js",
 names: ["bar", "baz", "n"],
 sources: ["one.js", "two.js"],
 mappings:
 "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}"##;

// exports.testMapEmptySourceRoot = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz", "n"],
// sources: ["one.js", "two.js"],
// sourceRoot: "",
// mappings:
// "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
// };
pub(crate) const testMapEmptySourceRoot: &str = r##"{
 version: 3,
 file: "min.js",
 names: ["bar", "baz", "n"],
 sources: ["one.js", "two.js"],
 sourceRoot: "",
 mappings:
 "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}"##;

// exports.testMapSingleSource = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz"],
// sources: ["one.js"],
// sourceRoot: "",
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
// };
pub(crate) const testMapSingleSource: &str = r##"
{
 version: 3,
 file: "min.js",
 names: ["bar", "baz"],
 sources: ["one.js"],
 sourceRoot: "",
 mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
}
"##;

// exports.testMapEmptyMappings = {
// version: 3,
// file: "min.js",
// names: [],
// sources: ["one.js", "two.js"],
// sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
// sourceRoot: "",
// mappings: ""
// };
pub(crate) const testMapEmptyMappings: &str = r##"
{
 version: 3,
 file: "min.js",
 names: [],
 sources: ["one.js", "two.js"],
 sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
 sourceRoot: "",
 mappings: ""
}
"##;

// exports.testMapEmptyMappingsRelativeSources = {
// version: 3,
// file: "min.js",
// names: [],
// sources: ["./one.js", "./two.js"],
// sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
// sourceRoot: "/the/root",
// mappings: ""
// };
pub(crate) const testMapEmptyMappingsRelativeSources: &str = r##"
{
 version: 3,
 file: "min.js",
 names: [],
 sources: ["./one.js", "./two.js"],
 sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
 sourceRoot: "/the/root",
 mappings: ""
}
"##;

// exports.testMapEmptyMappingsRelativeSources_generatedExpected = {
// version: 3,
// file: "min.js",
// names: [],
// sources: ["one.js", "two.js"],
// sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
// sourceRoot: "/the/root",
// mappings: ""
// };
pub(crate) const testMapEmptyMappingsRelativeSources_generatedExpected: &str = r##"
{
 version: 3,
 file: "min.js",
 names: [],
 sources: ["one.js", "two.js"],
 sourcesContent: [" ONE.foo = 1;", " TWO.inc = 2;"],
 sourceRoot: "/the/root",
 mappings: ""
}
"##;

// exports.testMapMultiSourcesMappingRefersSingleSourceOnly = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz"],
// sources: ["one.js", "withoutMappings.js"],
// sourceRoot: "",
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
// };
pub(crate) const testMapMultiSourcesMappingRefersSingleSourceOnly: &str = r##"
{
 version: 3,
 file: "min.js",
 names: ["bar", "baz"],
 sources: ["one.js", "withoutMappings.js"],
 sourceRoot: "",
 mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID"
}
"##;

// // This mapping is identical to above, but uses the indexed format instead.
// exports.indexedTestMap = {
// version: 3,
// file: "min.js",
// sections: [
// {
// offset: {
// line: 0,
// column: 0
// },
// map: {
// version: 3,
// sources: ["one.js"],
// sourcesContent: [
// " ONE.foo = function (bar) {\n   return baz(bar);\n };"
// ],
// names: ["bar", "baz"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
// file: "min.js",
// sourceRoot: "/the/root"
// }
// },
// {
// offset: {
// line: 1,
// column: 0
// },
// map: {
// version: 3,
// sources: ["two.js"],
// sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
// names: ["n"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
// file: "min.js",
// sourceRoot: "/the/root"
// }
// }
// ]
// };
pub(crate) const indexedTestMap: &str = r##"
{
 version: 3,
 file: "min.js",
 sections: [
    {
        offset: {
            line: 0,
            column: 0
        },
        map: {
            version: 3,
            sources: ["one.js"],
            sourcesContent: [
                " ONE.foo = function (bar) {\n   return baz(bar);\n };"
            ],
            names: ["bar", "baz"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
            file: "min.js",
            sourceRoot: "/the/root"
        }
    },
    {
         offset: {
            line: 1,
            column: 0
         },
        map: {
            version: 3,
            sources: ["two.js"],
            sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
            names: ["n"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
            file: "min.js",
            sourceRoot: "/the/root"
        }
    }
 ]
};
"##;

// exports.indexedTestMapDifferentSourceRoots = {
// version: 3,
// file: "min.js",
// sections: [
// {
// offset: {
// line: 0,
// column: 0
// },
// map: {
// version: 3,
// sources: ["one.js"],
// sourcesContent: [
// " ONE.foo = function (bar) {\n   return baz(bar);\n };"
// ],
// names: ["bar", "baz"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
// file: "min.js",
// sourceRoot: "/the/root"
// }
// },
// {
// offset: {
// line: 1,
// column: 0
// },
// map: {
// version: 3,
// sources: ["two.js"],
// sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
// names: ["n"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
// file: "min.js",
// sourceRoot: "/different/root"
// }
// }
// ]
// };
pub(crate) const indexedTestMapDifferentSourceRoots: &str = r##"
{
 version: 3,
 file: "min.js",
 sections: [
     {
        offset: {
            line: 0,
            column: 0
        },
        map: {
            version: 3,
            sources: ["one.js"],
            sourcesContent: [
                " ONE.foo = function (bar) {\n   return baz(bar);\n };"
            ],
            names: ["bar", "baz"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
            file: "min.js",
            sourceRoot: "/the/root"
        }
     },
    {
        offset: {
            line: 1,
            column: 0
        },
        map: {
            version: 3,
            sources: ["two.js"],
            sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
            names: ["n"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
            file: "min.js",
            sourceRoot: "/different/root"
        }
    }
 ]
}
"##;

// exports.indexedTestMapColumnOffset = {
// version: 3,
// file: "min.js",
// sections: [
// {
// offset: {
// line: 0,
// column: 0
// },
// map: {
// version: 3,
// sources: ["one.js"],
// sourcesContent: [
// " ONE.foo = function (bar) {\n   return baz(bar);\n };"
// ],
// names: ["bar", "baz"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
// file: "min.js",
// sourceRoot: "/the/root"
// }
// },
// {
// offset: {
// line: 0,
// // Previous section's last generated mapping is [32, Infinity), so
// // we're placing this a bit after that.
// column: 50
// },
// map: {
// version: 3,
// sources: ["two.js"],
// sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
// names: ["n"],
// mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
// file: "min.js",
// sourceRoot: "/the/root"
// }
// }
// ]
// };
pub(crate) const indexedTestMapColumnOffset: &str = r##"
{
 version: 3,
 file: "min.js",
 sections: [
    {
        offset: {
            line: 0,
            column: 0
        },
        map: {
            version: 3,
            sources: ["one.js"],
            sourcesContent: [
                " ONE.foo = function (bar) {\n   return baz(bar);\n };"
            ],
            names: ["bar", "baz"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID",
            file: "min.js",
            sourceRoot: "/the/root"
        }
    },
    {
        offset: {
            line: 0,
            // Previous section's last generated mapping is [32, Infinity), so
            // we're placing this a bit after that.
            column: 50
        },
        map: {
            version: 3,
            sources: ["two.js"],
            sourcesContent: [" TWO.inc = function (n) {\n   return n + 1;\n };"],
            names: ["n"],
            mappings: "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOA",
            file: "min.js",
            sourceRoot: "/the/root"
        }
    }
 ]
}
"##;

// exports.testMapWithSourcesContent = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz", "n"],
// sources: ["one.js", "two.js"],
// sourcesContent: [
// " ONE.foo = function (bar) {\n   return baz(bar);\n };",
// " TWO.inc = function (n) {\n   return n + 1;\n };"
// ],
// sourceRoot: "/the/root",
// mappings:
// "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
// };
pub(crate) const testMapWithSourcesContent: &str = r##"
{
 version: 3,
 file: "min.js",
 names: ["bar", "baz", "n"],
 sources: ["one.js", "two.js"],
 sourcesContent: [
 " ONE.foo = function (bar) {\n   return baz(bar);\n };",
 " TWO.inc = function (n) {\n   return n + 1;\n };"
 ],
 sourceRoot: "/the/root",
 mappings:
 "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}
"##;

// exports.testMapRelativeSources = {
// version: 3,
// file: "min.js",
// names: ["bar", "baz", "n"],
// sources: ["./one.js", "./two.js"],
// sourcesContent: [
// " ONE.foo = function (bar) {\n   return baz(bar);\n };",
// " TWO.inc = function (n) {\n   return n + 1;\n };"
// ],
// sourceRoot: "/the/root",
// mappings:
// "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
// };
pub(crate) const testMapRelativeSources: &str = r##"
{
 version: 3,
 file: "min.js",
 names: ["bar", "baz", "n"],
 sources: ["./one.js", "./two.js"],
 sourcesContent: [
 " ONE.foo = function (bar) {\n   return baz(bar);\n };",
 " TWO.inc = function (n) {\n   return n + 1;\n };"
 ],
 sourceRoot: "/the/root",
 mappings:
 "CAAC,IAAI,IAAM,SAAUA,GAClB,OAAOC,IAAID;CCDb,IAAI,IAAM,SAAUE,GAClB,OAAOA"
}
"##;

// exports.emptyMap = {
// version: 3,
// file: "min.js",
// names: [],
// sources: [],
// mappings: ""
// };
pub(crate) const emptyMap: &str = r##"
{
 version: 3,
 file: "min.js",
 names: [],
 sources: [],
 mappings: ""
}
"##;

// exports.mapWithSourcelessMapping = {
// version: 3,
// file: "example.js",
// names: [],
// sources: ["example.js"],
// mappings: "AAgCA,C"
// };
pub(crate) const mapWithSourcelessMapping: &str = r##"
{
 version: 3,
 file: "example.js",
 names: [],
 sources: ["example.js"],
 mappings: "AAgCA,C"
}
"##;
