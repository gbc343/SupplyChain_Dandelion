Item Supply Tracking Application



1. What is the idea and what is its purpose?

The purpose of this contract is to enable parties involved in a supply chain to track the movement and location of goods through the chain, from the manufacturer to the end consumer. This is a common use case that fits well into the platform's capabilities. The idea is using a blockchain-based supply chain contract to manage the system it can ensure the integrity of the supply chain, prevent theft, increase transparency, and increase efficiency in the supply chain process.



2. Describe the users of the smart contract and who are the users of the smart contract?

The main users of this contract will be manufacturers that will produce the initial items along with occupying distributers and shipping companies that manage the items transportation globally. In addition, retail outlets that sell the items will also be a major user of this smart contract. Users will have different information access and role depending on what their role is in the overall system.



3. In what fashion is each player going to interact with the smart contract? For example: "Bank teller: accesses the proprietary desktop dApp using his/her account wallet as the security key"

Manufacturers: The manufacturer would interact with the smart contract by adding latest items to the supply chain. This could be done through the system interface that is connected to the blockchain network and authenticated with a private key. Thus, manufacturers will be the initial point of entry of respective items in the system and through which all incoming data will be built upon.

Shipping companies: The shipping companies would interact with the smart contract by verifying the authenticity and movement of the goods in transit through scanning devices such as bar codes. Any irregularities or changes in item status could then be updated by the shippers that will be made available on the overall chain.

Retailers: The retailer would interact with the smart contract by verifying the authenticity of the goods they receive from the distributor. This could be done by scanning the bar code or unique identifier associated with each item to ensure that it matches the information stored on the blockchain.



4. Implementation

Since I was not familiar with Rust in general, I cobbled this contract together from various templates I found with a few changes of my own. It provides basic functionality of adding an item to the chain and getting an item to view. 
