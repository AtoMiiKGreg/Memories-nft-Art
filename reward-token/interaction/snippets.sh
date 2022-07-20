WALLET="walletKey.pem" # PEM path
ADDRESS=$(erdpy data load --key=address-devnet)
DEPLOY_TRANSACTION=$(erdpy data load --key=deployTransaction-devnet)
PROXY=https://devnet-gateway.elrond.com
CHAIN_ID="D"
WASM_PATH=output/reward-token.wasm


#string to hexa=0x$(xxd -pu <<< "arguments")
#integer to hexa=0x$(printf '%x\n' arguments)

# source snippets.sh && deploy

deploy() {

    erdpy --verbose contract deploy --recall-nonce --bytecode=${WASM_PATH} --pem=${WALLET} \
    --gas-limit=100000000 \
    --send --outfile="deploy-devnet.interaction.json" --proxy=${PROXY} --chain=${CHAIN_ID} || return

    TRANSACTION=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['emittedTransactionHash']")
    ADDRESS=$(erdpy data parse --file="deploy-devnet.interaction.json" --expression="data['contractAddress']")

    erdpy data store --key=address-devnet --value="${ADDRESS}"
    erdpy data store --key=deployTransaction-devnet --value="${TRANSACTION}"

    echo ""
    echo "Smart contract address: ${ADDRESS}"
}

issueToken() {
    local TOKEN_DISPLAY_NAME=TEST
    local TOKEN_TICKER=BRD


    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=600000000 --value=50000000000000000 --function="issueToken" \
    --arguments str:${TOKEN_DISPLAY_NAME} str:${TOKEN_TICKER}  \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

setLocalRoles() {
    erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
    --gas-limit=100000000 --function="setLocalRoles" \
    --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

mint(){
  # shellcheck disable=SC2162
  read -p "enter number: " AMOUNT

  erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
  --gas-limit=600000000 --function="mint" \
  --arguments "${AMOUNT}" \
  --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

burn(){
  # shellcheck disable=SC2162
  read -p "enter number: " AMOUNT

  erdpy --verbose contract call "${ADDRESS}" --recall-nonce --pem=${WALLET} \
  --gas-limit=600000000 --function="burn" \
  --arguments "${AMOUNT}" \
  --send --proxy=${PROXY} --chain=${CHAIN_ID}
}

upgradeSC() {
    erdpy --verbose contract upgrade "${ADDRESS}" --recall-nonce \
        --bytecode=${WASM_PATH} \
        --pem=${WALLET} \
        --gas-limit=60000000 \
        --proxy=${PROXY} --chain=${CHAIN_ID} \
        --send || return
}

