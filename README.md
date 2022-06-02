# TLC Main Bridge Contract

TLC Cross-Chain Bridge Contract Elrond Chain

# Smart Contract Developer - David Tanaka
Smart Contract Address : 

you can found on 

# Security / Functionality Audit - White Hat Cyber & TE





# Interaction

## On devnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=./testnet/wallets/users/alice.pem --proxy=http://localhost:7950 --contract=erd1...
```

## On testnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://testnet-gateway.elrond.com --contract=erd1...
```
## On mainnet

Deploy & interact with contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://gateway.elrond.com
```

Interact with existing contract:

```
python3 ./interaction/playground.py --pem=my.pem --proxy=https://gateway.elrond.com --contract=erd1...
