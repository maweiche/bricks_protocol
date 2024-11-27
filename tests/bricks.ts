import * as anchor from "@coral-xyz/anchor";
import { Program } from "@coral-xyz/anchor";
import { Bricks } from "../target/types/bricks";
import { PublicKey, Keypair } from "@solana/web3.js";
import { getAssociatedTokenAddressSync } from "@solana/spl-token";

describe("bricks", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env());

  const program = anchor.workspace.Bricks as Program<Bricks>;
  const wallet = anchor.Wallet.local();

  const protocol = PublicKey.findProgramAddressSync([Buffer.from("protocol")], program.programId)[0];
  const manager = PublicKey.findProgramAddressSync([Buffer.from("manager")], program.programId)[0];

  /////-- INITIALIZATION --////////
  ////---- ONLY RUN ONCE ----/////
  // it("Initialize Protocol", async () => {
  //   console.log('init protocol')
  //   await program.methods.initializeProtocol()
  //   .accountsPartial({  
  //     owner: wallet.publicKey,    
  //     protocol,
  //     manager,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .signers([wallet.payer])
  //   .rpc();
  // });

  const adminProfile = PublicKey.findProgramAddressSync([Buffer.from("admin"), wallet.publicKey.toBuffer()], program.programId)[0];
  const adminArgs = {
    username: "matt"
  }
  const _newAdmin = new PublicKey('7wK3jPMYjpZHZAghjersW6hBNMgi9VAGr75AhYRqR2n');

  ////-- CREATE NEW ADMIN --//////
  ////---- ONLY RUN ONCE ----/////
  it("Initialize Admin", async () => {
    await program.methods.initializeAdmin(adminArgs)
    .accountsPartial({  
      owner: wallet.publicKey,    
      newAdmin: wallet.publicKey,
    })
    .signers([wallet.payer])
    .rpc();
  });

  /////-- LOCKS PROTOCOL --////////
  ////---- RENDERS PROTOCOL INOPERABLE ----/////
  // it("Update Protocol", async () => {
  //   await program.methods.updateProtocol()
  //   .accountsPartial({  
  //     owner: wallet.publicKey,    
  //     protocol,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .signers([wallet.payer])
  //   .rpc();
  // });

  const profile = PublicKey.findProgramAddressSync([Buffer.from("profile"), wallet.publicKey.toBuffer()], program.programId)[0];
  const profileArgs = {
    username: "Matt"
  }

  ////-- CREATE NEW PROFILE --//////
  ////---- USER MUST HAVE A PROFILE ----/////
  ////---- TO PURCHASE BRICKS ----/////
  // it("Initialize Profile", async () => {
  //   await program.methods.initializeProfile(profileArgs)
  //   .accountsPartial({
  //     user: wallet.publicKey,  
  //     payer: wallet.publicKey,    
  //     profile,
  //     systemProgram: anchor.web3.SystemProgram.programId,
  //   })
  //   .signers([wallet.payer])
  //   .rpc();
  // });

  // HOUSE TO BE CREATED
  const createObjectArgs = {
    name: "Luxury Condo - Miami, FL",
    uri: "https://arweave.net/G7r27Nw0A3jH0fdFkuh1xqqSaJvtU4HQoN_R-X6Y-is",
    reference: "25.766,-80.132", // GPS Coordinates of Property
    attributes: [
      { attributeList: [
        { key: "bedrooms", value: "3" },
        { key: "bathrooms", value: "2" },
        { key: "sqft", value: "1200" },
        { key: "price", value: "900000" },
        { key: "type", value: "Condo" },
        { key: "location", value: "Miami, FL" },
        { key: "yearBuilt", value: "2020" }
      ]}
    ],
  };

  // Home to be created
  const homePda = PublicKey.findProgramAddressSync([Buffer.from("object"), Buffer.from(createObjectArgs.reference)], program.programId)[0];

  const mplCoreProgram = new PublicKey("CoREENxT6tW1HoK8ypY1SxRMZTcVPm7R94rH4PZNhX7d")

  ////-- CREATE NEW OBJECT --//////
  ////---- THIS WILL WHAT WE WILL ----/////
  ////---- CREATE LISTING/FRACTIONS FROM ----/////
  it("Creates a Object", async () => {
    await program.methods.createObject(createObjectArgs)
    .accountsPartial({
      admin: wallet.publicKey,
      adminProfile,
      manager,
      protocol,
      object: homePda,
      mplCoreProgram,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([wallet.payer])
    .rpc({skipPreflight: true});
  });

  ////-- CREATE FRACITONALIZED LISTING --//////
  ////---- SETS PARAMS FOR FRACTIONALIZATION ----/////
  ////---- A HOME CAN HAVE MULTIPLE LISTINGS ----/////
  const createFractionalizedListingArgs = {
    id: new anchor.BN(Math.floor(Math.random() * 1000000)),
    objectType: 0,
    share: 100,
    price: new anchor.BN(1),
    startingTime: new anchor.BN(Math.floor(Date.now() / 1000) - 7 * 24 * 60 * 60)
  };
  const mint = new PublicKey('4zMMC9srt5Ri5X14GAgXhaHii3GnPAEERYPJgZJDncDU');
  console.log('id -> ', createFractionalizedListingArgs.id.toNumber())
  const _id = createFractionalizedListingArgs.id.toNumber();
  let listing = PublicKey.findProgramAddressSync([Buffer.from("listing"), createFractionalizedListingArgs.id.toBuffer("le", 8)], program.programId)[0];
  console.log('id -> ', createFractionalizedListingArgs.id.toNumber())
  // let listing = new PublicKey('7J4McGtP4WM8s3riHZCCLYE9nRScedQDzsJsevn4pnjY');
  console.log('listing', listing.toBase58())

  it("Creates Fractionalized Listing", async () => {
    await program.methods.createFractionalizedListing(createFractionalizedListingArgs)
    .accountsPartial({
      admin: wallet.publicKey,
      adminProfile,
      manager,
      object: homePda,
      listing,
      systemProgram: anchor.web3.SystemProgram.programId,
    })
    .signers([wallet.payer])
    .rpc();
  });
  const listingAta = getAssociatedTokenAddressSync(mint, listing, true);
  const fraction = Keypair.generate();

  it("Buy Fractionalized Listing", async () => {
    await program.methods.buyFractionalizedListing("https://example.com")
    .accountsPartial({
      buyer: wallet.publicKey,
      payer: wallet.publicKey,
      mint: mint,
      listing: listing,
      object: homePda,
      fraction: fraction.publicKey,
    })
    .signers([wallet.payer, fraction])
    .rpc();
  });
});
