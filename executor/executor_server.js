var PROTO_PATH = __dirname + '/../service/proto/src/proto/executor/v1/executor.proto';

var grpc = require('@grpc/grpc-js');
const { log } = require('@grpc/grpc-js/build/src/logging');
var protoLoader = require('@grpc/proto-loader');
var packageDefinition = protoLoader.loadSync(
    PROTO_PATH,
    {keepCase: true,
     longs: String,
     enums: String,
     defaults: true,
     oneofs: true
    });
var executor_proto = grpc.loadPackageDefinition(packageDefinition).executor.v1;

/**
 * Implements the ProcessBatch RPC method.
 */
function ProcessBatch(call, callback) {
  console.log("ProcessBatch request: ", call.request)
  let ROM_ERROR_NO_ERROR = 1
  let EXECUTOR_ERROR_NO_ERROR = 1
  let responses_logs = {
    address: "address",
    topics: [Buffer.from("topics")],
    data: Buffer.from("data"),
    batch_number: 0,
    tx_hash: Buffer.from("tx_hash"),
    tx_index: 0,
    batch_hash: Buffer.from("batch_hash"),
    index: 0
  }

  let executionTraceStep = {
    pc: 0,
    op: "op",
    remaining_gas: 0,
    gas_cost: 0,
    memory: 0,
    memory_size: 0,
    memory_offset: 0,
    stack: ["stack"],
    return_data: Buffer.from("return_data"),
    storage: {
      "storage": "storage"
    },
    depth: 0,
    gas_refund: 0,
    error: ROM_ERROR_NO_ERROR
  }

  let transactionContext = {
    type: "type",
    from: "from",
    to: "to",
    data: Buffer.from("data"),
    gas: 0,
    value: "value",
    batch: Buffer.from("batch"),
    output: Buffer.from("output"),
    gas_used: 0,
    gas_price: "0",
    execution_time: 0,
    old_state_root: Buffer.from("old_state_root")
  }

  let contract = {
    address: "address",
    caller: "caller",
    value: "value",
    data: Buffer.from("data"),
    gas: 0
  }
  let transactionStep = {
    state_root: Buffer.from("state_root"),
    depth: 0,
    pc: 0,
    gas: 0,
    gas_cost: 0,
    gas_refund: 0,
    op: 0,
    stack: ["stack"],
    memory: Buffer.from("memory"),
    memory_size: 0,
    memory_offset: 0,
    return_data: Buffer.from("return_data"),
    contract: contract,
    error: ROM_ERROR_NO_ERROR
  }

  let call_trace = {
    context: transactionContext,
    steps: [transactionStep]
  }
  let processTransactionResponse = {
    tx_hash: Buffer.from("tx_hash"),
    rlp_tx: Buffer.from("rlp_tx"),
    type: 0,
    return_value: Buffer.from("return_value"),
    gas_left: 0,
    gas_used: 0,
    gas_refunded: 0,
    error: ROM_ERROR_NO_ERROR,
    create_address: "create_address",
    state_root: Buffer.from("state_root"),
    logs: [responses_logs],
    execution_trace: [executionTraceStep],
    call_trace: call_trace
  }

  let read_write_addresses = {
    "read_write_addresses": {
      nonce: 0,
      balance: 0
    }
  }
  processBatchResponse = {
    new_state_root: Buffer.from('new_state_root'),
    new_acc_input_hash: Buffer.from('new_acc_input_hash'),
    new_local_exit_root: Buffer.from('new_local_exit_root'),
    new_batch_num: 0,
    cnt_keccak_hashes: 0,
    cnt_poseidon_hashes: 0,
    cnt_poseidon_paddings: 0,
    cnt_mem_aligns: 0,
    cnt_arithmetics: 0,
    cnt_binaries: 0,
    cnt_steps: 0,
    cumulative_gas_used: 0,
    responses: [processTransactionResponse],
    error: EXECUTOR_ERROR_NO_ERROR,
    read_write_addresses: read_write_addresses
  }
  callback(null, processBatchResponse);
}

/**
 * Starts an RPC server that receives requests for the Executor service at the
 * sample server port
 */
function main() {
  var server = new grpc.Server();
  server.addService(executor_proto.ExecutorService.service, {ProcessBatch: ProcessBatch});
  console.log("executor service is running")
  server.bindAsync('0.0.0.0:50051', grpc.ServerCredentials.createInsecure(), () => {
    server.start();
  });
}

main();
