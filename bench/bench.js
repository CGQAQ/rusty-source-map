const fs = require("fs").promises;
const consumer = require("./fixture/lib/source-map-consumer");

(async () => {
	const content = (await fs.readFile("./angular-min-source-map.json")).toString();
	console.time("bench");
	const c = await new consumer.SourceMapConsumer(content);
	c.eachMapping(() => { }, c, consumer.SourceMapConsumer.GENERATED_ORDER);
	console.timeEnd("bench");
	c.destroy();
})()
