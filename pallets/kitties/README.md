## Exercise 02: Create Pallet-kitties 



<details>
<summary> EXERCISE 02 : Create Pallet Kitties </summary>
  
  <details>
  <summary> STEP 1 </summary>
  <strong>Define 3 on-chain storages</strong>
  <ul>
    <li> KittyId : StorageValue </li>
    <li> Kitties : StorageMap(DNA => Kitty) </li>
    <li>KittiesOwned : StorageMap(AccountId => Vec(DNA))</li>
  </ul>

  </details>

  <details>
  <summary> STEP 2 </summary>
    <strong>Define Event and Error (There are some suggestions in the code section)</strong>
  </details>
    <details>
  <summary> STEP 3 </summary>
    <strong>Create helper function `gen_dna`:</strong>
      <ul>
    <li> Input : DNA  </li>
    <li> if dna length is even return Male else return Female </li>
    <li>Output: Result(Gender, Error(T)))</li>
  </ul>
  </details>

  <details>
  <summary> STEP 4 </summary>
  <strong>Create extrinsic call `create_kitty`</strong>
  <ul>
    <li>generate gender and declair new kitty</li>
    <li>Check duplicate kitty, 1 kitty has different dna</li>
    <li> Get current kitty id </li>
    <li> Add new kitty to  on-chain storage KittiesOwned </li>
    <li> Write new kitty to storage Kitties and KittyId </li>
    <li> Emit Create Event </li>
  </ul>

  </details>
  <details>
  <summary> STEP 5 </summary>
    <strong>Verify whether your extrinsic call logic is correct or not. Remember to start your local node and access polkadotjs explorer</strong>
  <ul>
    <li>Create first kitty successfully</li>
    <img src="../../images/Create_first_kitty.png">
    <li>Access on-chain storage </li>
    <img src="../../images/Access_on_chain_storage.png">
    <li>Create second kitty successfully</li>
    <img src="../../images/Create_second_kitty.png">
    <li>Create duplicate kitty -> Emit DuplicateKitty Error </li>
    <img src="../../images/Duplicate_Kitty.png">
  </ul>

  </details>
</details>