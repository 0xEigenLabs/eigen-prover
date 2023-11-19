let PROTO_PATH =
  __dirname + "/../service/proto/src/proto/executor/v1/executor.proto";

let parseArgs = require("minimist");
let grpc = require("@grpc/grpc-js");
let protoLoader = require("@grpc/proto-loader");
let packageDefinition = protoLoader.loadSync(PROTO_PATH, {
  keepCase: true,
  longs: String,
  enums: String,
  defaults: true,
  oneofs: true,
});

let executor_proto = grpc.loadPackageDefinition(packageDefinition).executor.v1;

function main() {
  let argv = parseArgs(process.argv.slice(2), {
    string: "target",
  });
  let target;
  if (argv.target) {
    target = argv.target;
  } else {
    target = "localhost:50071";
  }
  let client = new executor_proto.ExecutorService(
    target,
    grpc.credentials.createInsecure()
  );
  let user;
  if (argv._.length > 0) {
    user = argv._[0];
  } else {
    user = "world";
  }

  let db = {
    db: "db",
  };

  let contracts_bytecode = {
    contracts_bytecode: "contracts_bytecode",
  };

  let trace_config = {
    disable_storage: 0,
    disable_stack: 0,
    enable_memory: 0,
    enable_return_data: 0,
    tx_hash_to_generate_execute_trace: Buffer.from(
      "tx_hash_to_generate_execute_trace"
    ),
    tx_hash_to_generate_call_trace: Buffer.from(
      "tx_hash_to_generate_call_trace"
    ),
  };
  let processBatchRequest = {
    old_state_root: Buffer.from("old_state_root"),
    old_acc_input_hash: Buffer.from("old_acc_input_hash"),
    old_batch_num: 0,
    chain_id: 0,
    fork_id: 0,
    batch_l2_data: Buffer.from("[1, 2]"),
    global_exit_root: Buffer.from("global_exit_root"),
    eth_timestamp: 0,
    coinbase: "coinbase",
    update_merkle_tree: 0,
    no_counters: 0,
    from: "from",
    db: db,
    contracts_bytecode: contracts_bytecode,
    trace_config: trace_config,
  };
  // console.log("processBatchRequest:", processBatchRequest)
  client.ProcessBatch(processBatchRequest, function (err, response) {
    console.log("res:", response);
  });
}

main();
