use crate::constants::*;
use crate::consumer::*;

// exports[
//   "test that we can instantiate with a string or an object"
// ] = async function(assert) {
//   let map = await new SourceMapConsumer(util.testMap);
//   map = await new SourceMapConsumer(JSON.stringify(util.testMap));
//   assert.ok(true);
//   map.destroy();
// };
#[test]
fn test_instantiate() {
    let _ = create_consumer(testMap).unwrap();
}

// exports[
//   "test that the object returned from await new SourceMapConsumer inherits from SourceMapConsumer"
// ] = async function(assert) {
//   const map = await new SourceMapConsumer(util.testMap);
//   assert.ok(map instanceof SourceMapConsumer);
//   map.destroy();
// };
#[test]
fn test_is_consumer() {
    assert!(create_consumer(testMap).is_ok())
}

// exports[
//   "test that a BasicSourceMapConsumer is returned for sourcemaps without sections"
// ] = async function(assert) {
//   const map = await new SourceMapConsumer(util.testMap);
//   assert.ok(map instanceof BasicSourceMapConsumer);
//   map.destroy();
// };
#[test]
fn test_no_section() {
    let map = create_consumer(testMap).unwrap();
    if let Consumer::BasicConsumer(_) = map {
        assert!(true)
    } else {
        assert!(false)
    }
}

// exports[
//   "test that an IndexedSourceMapConsumer is returned for sourcemaps with sections"
// ] = async function(assert) {
//   const map = await new SourceMapConsumer(util.indexedTestMap);
//   assert.ok(map instanceof IndexedSourceMapConsumer);
//   map.destroy();
// };
#[test]
fn test_has_section() {
    let map = create_consumer(indexedTestMap).unwrap();
    if let Consumer::IndexedConsumer(_) = map {
    } else {
        unreachable!()
    }
}

// exports[
//   "test that the `sources` field has the original sources"
// ] = async function(assert) {
//   let map;
//   let sources;
//
//   map = await new SourceMapConsumer(util.testMap);
//   sources = map.sources;
//   assert.equal(sources[0], "/the/root/one.js");
//   assert.equal(sources[1], "/the/root/two.js");
//   assert.equal(sources.length, 2);
//   map.destroy();
//
//   map = await new SourceMapConsumer(util.indexedTestMap);
//   sources = map.sources;
//   assert.equal(sources[0], "/the/root/one.js");
//   assert.equal(sources[1], "/the/root/two.js");
//   assert.equal(sources.length, 2);
//   map.destroy();
//
//   map = await new SourceMapConsumer(util.indexedTestMapDifferentSourceRoots);
//   sources = map.sources;
//   assert.equal(sources[0], "/the/root/one.js");
//   assert.equal(sources[1], "/different/root/two.js");
//   assert.equal(sources.length, 2);
//   map.destroy();
//
//   map = await new SourceMapConsumer(util.testMapNoSourceRoot);
//   sources = map.sources;
//   assert.equal(sources[0], "one.js");
//   assert.equal(sources[1], "two.js");
//   assert.equal(sources.length, 2);
//   map.destroy();
//
//   map = await new SourceMapConsumer(util.testMapEmptySourceRoot);
//   sources = map.sources;
//   assert.equal(sources[0], "one.js");
//   assert.equal(sources[1], "two.js");
//   assert.equal(sources.length, 2);
//   map.destroy();
// };
#[test]
fn test_sources_has_original_sources() {
    let map = create_consumer(testMap).unwrap();
    let map = map.as_basic_consumer();
    assert_eq!(map.get_sources()[0], "/the/root/one.js");
    assert_eq!(map.get_sources()[1], "/the/root/two.js");
    assert_eq!(map.get_sources().len(), 2);
}

#[test]
fn test_sources() {
    let map = create_consumer(testMap).unwrap();
    if let Consumer::BasicConsumer(consumer) = map {
        let sources = consumer.absolute_sources.to_vec();
        assert_eq!(sources[0], "/the/root/one.js");
        assert_eq!(sources[1], "/the/root/two.js");
        assert_eq!(sources.len(), 2);
        return;
    }
    panic!("Not ok");
}
