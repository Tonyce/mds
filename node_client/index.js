const net = require('net');

const { HelloRequest, HelloReply } = require('./proto/helloworld_pb');

const request = new HelloRequest();
request.setName("world");

var bytes = request.serializeBinary();

const data = encodeMsg(bytes)
console.log({ data })
const client = net.createConnection({ port: 2345 }, () => {
  // 'connect' listener.
  console.log('connected to server!');
  client.write(data);
});
client.on('data', (data) => {
  console.log(data.toString());
  // client.end();
});
client.on('end', () => {
  console.log('disconnected from server');
});

function encodeMsg(msgByte) {
  const msgByteLen = msgByte.length;
  const lenBuf = int2BufEndianWay(msgByteLen);
  return Buffer.concat([lenBuf, msgByte])
}

function int2BufEndianWay(x) {
  buf = Buffer.allocUnsafe(4)
  buf.writeUIntBE(x, 0, 4)
  return buf
}