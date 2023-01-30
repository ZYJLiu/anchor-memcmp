import * as anchor from "@project-serum/anchor"
import { Program } from "@project-serum/anchor"
import { bs58 } from "@project-serum/anchor/dist/cjs/utils/bytes"
import { expect } from "chai"
import { AnchorMemcmp } from "../target/types/anchor_memcmp"

describe("anchor-memcmp", () => {
  // Configure the client to use the local cluster.
  anchor.setProvider(anchor.AnchorProvider.env())

  const program = anchor.workspace.AnchorMemcmp as Program<AnchorMemcmp>

  it("Is initialized!", async () => {
    const tx = await program.methods.initialize("First", "Last").rpc()
  })

  it("First String", async () => {
    const name = bs58.encode(Buffer.from("F"))
    const accounts = await program.account.fullName.all([
      {
        memcmp: {
          offset: 8 + 4,
          bytes: name,
        },
      },
    ])
    for (const account of accounts) {
      expect(account.account.firstName).to.include("First")
      console.log(account.account.firstName)
    }
  })

  it("Second String", async () => {
    const name = bs58.encode(Buffer.from("Last"))
    const accounts = await program.account.fullName.all([
      {
        memcmp: {
          offset: 8 + 4 + 120 + 4,
          bytes: name,
        },
      },
    ])
    for (const account of accounts) {
      expect(account.account.firstName).to.include("Last")
      console.log(account.account.firstName)
    }
  })
})
