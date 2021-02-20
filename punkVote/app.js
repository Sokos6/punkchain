ethers = require('ethers')
var fs = require('fs')
bytecode = fs.readFileSync('Voting_sol_Voting.bin').toString()
abi = JSON.parse(fs.readFileSync('Voting_sol_Voting.abi').toString())
provider = new ethers.providers.JsonRpcProvider()
provider.listAccounts().then(result => console.log(result))
signer = provider.getSigner(0)
factory = new ethers.ContractFactory(abi, bytecode, signer)
contract = null
factory.deploy([ethers.utils.formatBytes32String('Will'), ethers.utils.formatBytes32String('Marmalade'), ethers.utils.formatBytes32String('Meepers')]).then((c) => { contract = c})
contract.totalVotesFor(ethers.utils.formatBytes32String('Wil')).then((f) => console.log(f.toNumber()))
contract.voteForCandidate(ethers.utils.formatBytes32String('Will')).then((f) => console.log(f))
contract.totalVotesFor(ethers.utils.formatBytes32String('Will')).then((f) => console.log(f.toNumber()))