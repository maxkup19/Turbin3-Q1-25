import * as anchor from "@coral-xyz/anchor";
import { AnchorError, Program, web3 } from "@coral-xyz/anchor";
import { PredictionMarket } from "../target/types/prediction_market";
import BN from "bn.js";
import { assert } from "chai";
import crypto from "crypto";

const provider = anchor.AnchorProvider.env();
anchor.setProvider(provider);

const program = anchor.workspace.PredictionMarket as Program<PredictionMarket>;

const admin = web3.Keypair.generate();

// Users
const creatorOne = web3.Keypair.generate();
const user1 = web3.Keypair.generate();
const user2 = web3.Keypair.generate();
const user3 = web3.Keypair.generate();

// Questions
const questionOne = "Question 1";
const questionEmpty = "";
const questionTwo = "Question 2";

// Create hash of the questions to use as seeds
const questionOneHash = crypto.hash("SHA256", questionOne);
const questionTwoHash = crypto.hash("SHA256", questionTwo);
const questionEmptyHash = crypto.hash("SHA256", questionEmpty);

const marketLength =  10;

// Program Config
const programConfigPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("config")],
    program.programId
)[0];
const vaultPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("vault"), programConfigPDA.toBuffer()],
    program.programId
)[0];

// Market
const marketOnePDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("market"), creatorOne.publicKey.toBuffer(), Buffer.from(questionOneHash, 'hex')],
    program.programId
)[0];
const marketTwoPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("market"), creatorOne.publicKey.toBuffer(), Buffer.from(questionTwoHash, 'hex')],
    program.programId
)[0];
const marketEmptyPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("market"), creatorOne.publicKey.toBuffer(), Buffer.from(questionEmptyHash, 'hex')],
    program.programId
)[0];

// Bet PDAs
const user1Market1PositiveBetPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet"), user1.publicKey.toBuffer(), marketOnePDA.toBuffer()],
    program.programId
)[0];

const user2Market1NegativeBetPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet"), user2.publicKey.toBuffer(), marketOnePDA.toBuffer()],
    program.programId
)[0];

const user3Market1PositiveBetPDA = web3.PublicKey.findProgramAddressSync(
    [Buffer.from("bet"), user3.publicKey.toBuffer(), marketOnePDA.toBuffer()],
    program.programId
)[0];

// Bets
const user1Market1PositiveBetAmount = new BN(0.5 * web3.LAMPORTS_PER_SOL);
const user2Market1NegativeBetAmount = new BN(0.25 * web3.LAMPORTS_PER_SOL);
const user3Market1PositiveBetAmount = new BN(0.75 * web3.LAMPORTS_PER_SOL);

// Init config
describe("Initialize Program Config", () => {
    before(async () => {
        await airdrop(program, admin, 2);
    });

    it("admin can initialize program config", async () => {
        await program.methods
        .initProgramConfig()
        .accountsStrict({
            authority: admin.publicKey,
            programConfig: programConfigPDA,
            vault: vaultPDA,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([admin])
        .rpc()
        .catch(err => {
            console.log(err);
            assert.ok(false);
        });

        const programConfigData = await program.account.programConfig.fetch(programConfigPDA);
        assert(programConfigData.authority.equals(admin.publicKey), "Program config authority should be admin");

        const vaultData = await provider.connection.getAccountInfo(vaultPDA);
        assert(vaultData, "Vault should exist");
    });
});

// Init market

describe("Initialize market", () => {


    // Airdrop some SOL to the creator for testing
    before(async () => {
        for (const keypair of [admin, creatorOne, user1, user2, user3]) {
            await airdrop(program, keypair, 2);
        }
    });

    it("Test user can create first market", async () => {
        await program.methods
            .initMarket(questionOne, new BN(Date.now() / 1000 + marketLength), Buffer.from(questionOneHash, 'hex'))
            .accountsStrict({
                creator: creatorOne.publicKey,
                marketState: marketOnePDA,
                programConfig: programConfigPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([creatorOne])
            .rpc()
            .catch(_ => assert.ok(false));

        const marketOneData = await program.account.marketState.fetch(marketOnePDA);
        assert(marketOneData.creator.equals(creatorOne.publicKey), "Market creator should be creator One");
        assert(marketOneData.noAmount.eq(new BN(0)), "Market creator should have 0 no amount");
        assert(marketOneData.yesAmount.eq(new BN(0)), "Market creator should have 0 yes amount");
        assert.strictEqual(marketOneData.question, questionOne);
        assert(marketOneData.outcome === null);
    });

    it("Can't create a market with no name", async () => {
        try {
            await program.methods
            .initMarket(questionEmpty, new BN(Date.now() / 1000 + marketLength), Buffer.from(questionEmptyHash, 'hex'))
            .accountsStrict({
                creator: creatorOne.publicKey,
                marketState: marketEmptyPDA,
                programConfig: programConfigPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([creatorOne])
            .rpc()
        } catch (e) {
            assert.equal(e.error.errorCode.number, 6000);
        };
    });

    it("creatorOne can init market can initialize second market", async () => {
        await program.methods
            .initMarket(questionTwo, new BN(Date.now() / 1000 + marketLength), Buffer.from(questionTwoHash, 'hex'))
            .accountsStrict({
                creator: creatorOne.publicKey,
                marketState: marketTwoPDA,
                programConfig: programConfigPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([creatorOne])
            .rpc()
            .catch(_ => assert.ok(false));
        const marketTwoData = await program.account.marketState.fetch(marketTwoPDA);

        assert(marketTwoData.creator.equals(creatorOne.publicKey));
        assert.strictEqual(marketTwoData.question, questionTwo);
    });
});

describe("Betting process", () => {
    it("user 1 can place a positive bet on market 1", async () => {
        await program.methods
            .placeBet({ positive: {} }, user1Market1PositiveBetAmount)
            .accountsStrict({
                signer: user1.publicKey,
                marketState: marketOnePDA,
                betState: user1Market1PositiveBetPDA,
                programConfig: programConfigPDA,
                vault: vaultPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user1])
            .rpc()
            .catch(e => {
                console.log(e);
                assert.ok(false);
            });

            const vaultInfo = await provider.connection.getAccountInfo(vaultPDA);
            const user1Market1PositiveBetData = await program.account.betState.fetch(user1Market1PositiveBetPDA);
            const marketOneData = await program.account.marketState.fetch(marketOnePDA);

            const vaultRent = await provider.connection.getMinimumBalanceForRentExemption(vaultInfo.data.length);
            console.log(vaultRent);
            const vaultDiff = vaultInfo.lamports - vaultRent;

            assert(vaultDiff == user1Market1PositiveBetAmount.toNumber(), "The vault should have the bet amount");
            assert(user1Market1PositiveBetData.owner.equals(user1.publicKey), "The owner of the bet should be user1");
            assert(user1Market1PositiveBetData.market.equals(marketOnePDA), "The market associated with the bet should be marketOne");
            assert(user1Market1PositiveBetData.amount.eq(user1Market1PositiveBetAmount), "The bet amount should match the expected amount");
            assert.deepEqual(user1Market1PositiveBetData.bet, { positive: {} }, "The bet type should be positive");
            assert(marketOneData.yesAmount.eq(user1Market1PositiveBetAmount), "The market should have the bet amount");
            assert(marketOneData.noAmount.eq(new BN(0)), "The market should have 0 no amount");
    });

    it("user can't place bet on the same market twice", async () => {
        try {
            await program.methods
            .placeBet({ positive: {} }, user1Market1PositiveBetAmount)
            .accountsStrict({
                signer: user1.publicKey,
                marketState: marketOnePDA,
                betState: user1Market1PositiveBetPDA,
                programConfig: programConfigPDA,
                vault: vaultPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user1])
            .rpc()

            assert.ok(false);
        } catch (e) {
            assert.ok(true);
        }

        try  {
            await program.methods
            .placeBet({ negative: {} }, user1Market1PositiveBetAmount)
            .accountsStrict({
                signer: user1.publicKey,
                marketState: marketOnePDA,
                betState: user1Market1PositiveBetPDA,
                programConfig: programConfigPDA,
                vault: vaultPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user1])
            .rpc()

            assert.ok(false);
        } catch (e) {
            assert.ok(true);
        }
    });

    it("user 2 can place a negative bet on market 1", async () => {
        await program.methods
        .placeBet({ negative: {} }, user2Market1NegativeBetAmount)
        .accountsStrict({
            signer: user2.publicKey,
            marketState: marketOnePDA,
            betState: user2Market1NegativeBetPDA,
            programConfig: programConfigPDA,
            vault: vaultPDA,
            systemProgram: web3.SystemProgram.programId,
        })
        .signers([user2])
        .rpc()
        .catch(e => {
            console.log(e);
            assert.ok(false);
        });

        const vaultInfo = await provider.connection.getAccountInfo(vaultPDA);
        const user2Market1NegativeBetData = await program.account.betState.fetch(user2Market1NegativeBetPDA);
        const marketOneData = await program.account.marketState.fetch(marketOnePDA);

        const vaultRent = await provider.connection.getMinimumBalanceForRentExemption(vaultInfo.data.length);
        const vaultDiff = vaultInfo.lamports - vaultRent - user1Market1PositiveBetAmount.toNumber();

        assert(vaultDiff == user2Market1NegativeBetAmount.toNumber(), "The vault should have the bet amount");
        assert(user2Market1NegativeBetData.owner.equals(user2.publicKey), "The owner of the bet should be user2");
        assert(user2Market1NegativeBetData.market.equals(marketOnePDA), "The market associated with the bet should be marketOne");
        assert.deepEqual(user2Market1NegativeBetData.bet, { negative: {} }, "The bet type should be negative");
        assert(vaultInfo.lamports - vaultRent == (marketOneData.yesAmount.add(marketOneData.noAmount)).toNumber(), "The market should have the bet amount");
    });

    it("user 3 can place a positive bet on market 1", async () => {
        await program.methods
            .placeBet({ positive: {} }, user3Market1PositiveBetAmount)
            .accountsStrict({
                signer: user3.publicKey,
                marketState: marketOnePDA,
                betState: user3Market1PositiveBetPDA,
                programConfig: programConfigPDA,
                vault: vaultPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user3])
            .rpc()
            .catch(e => {
                console.log(e);
                assert.ok(false);
            });
    });
});

describe("Resolve market", () => {
    it("Unauthorized user can't resolve market", async () => {
        try {
            await program.methods
            .resolveMarket({ positive: {} })
            .accountsStrict({
                admin: admin.publicKey,
                marketState: marketOnePDA,
                programConfig: programConfigPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user1])
            .rpc()

            assert.ok(false);
        } catch (error) {
            assert.ok(true);
        }
    });

    it("Admin can't resolve market before expiration", async () => {
        try {
            await program.methods
                .resolveMarket({ positive: {} })
                .accountsStrict({
                    admin: admin.publicKey,
                    marketState: marketOnePDA,
                    programConfig: programConfigPDA,
                    systemProgram: web3.SystemProgram.programId,
                })
                .signers([admin])
                .rpc()

            assert.ok(false);
        } catch (error) {
            assert.ok(true);
        }
    });

    it("Admin can resolve market after expiration", async () => {

        // TODO: Add code to advance clock or wait for market expiration
        await new Promise(resolve => setTimeout(resolve, marketLength * 1000));

        await program.methods
            .resolveMarket({ positive: {} })
            .accountsStrict({
                admin: admin.publicKey,
                marketState: marketOnePDA,
                programConfig: programConfigPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([admin])
            .rpc()
            .catch(err => {
                console.log(err);
                assert.ok(false)
            });

        const marketOneData = await program.account.marketState.fetch(marketOnePDA);
        assert(marketOneData.outcome !== null, "Market should be resolved");
        assert.deepEqual(marketOneData.outcome, {positive: {}}, "Market should be resolved to positive");

    });
});

describe("Withdraw", () => {
    it("user 1 can withdraw their bet", async () => {
        const vaultInfo = await provider.connection.getAccountInfo(vaultPDA);
        const vaultRent = await provider.connection.getMinimumBalanceForRentExemption(vaultInfo.data.length);
        const vaultLamportsBefore = (await provider.connection.getAccountInfo(vaultPDA)).lamports - vaultRent;

        await program.methods
            .withdraw()
            .accountsStrict({
                user: user1.publicKey,
                marketState: marketOnePDA,
                betState: user1Market1PositiveBetPDA,
                programConfig: programConfigPDA,
                vault: vaultPDA,
                systemProgram: web3.SystemProgram.programId,
            })
            .signers([user1])
            .rpc()
            .catch(e => {
                console.log(e);
                assert.ok(false);
            });

        const vaultLamportsAfter = (await provider.connection.getAccountInfo(vaultPDA)).lamports - vaultRent;
        const user1Market1PositiveBetData = await provider.connection.getAccountInfo(user1Market1PositiveBetPDA);
        const marketOneData = await program.account.marketState.fetch(marketOnePDA);
        const total_amount = marketOneData.yesAmount.add(marketOneData.noAmount);
        const user_share = user1Market1PositiveBetAmount.toNumber() / marketOneData.yesAmount.toNumber();
        const profit = Math.floor(user_share * total_amount.toNumber()) - user1Market1PositiveBetAmount.toNumber();
        const user_win = user1Market1PositiveBetAmount.toNumber() + (profit > 0 ? Math.floor(profit * 95 / 100) : 0);

        assert(user1Market1PositiveBetData === null, "The bet should be closed");
        assert(vaultLamportsAfter == vaultLamportsBefore - user_win, "The vault should have the bet amount");
    });
});

async function airdrop(
    program: anchor.Program<PredictionMarket>,
    keypair: web3.Keypair,
    amount: number
) {
    const airdropSignature = await program.provider.connection.requestAirdrop(
        keypair.publicKey,
        amount * anchor.web3.LAMPORTS_PER_SOL
    );

    await program.provider.connection.confirmTransaction(airdropSignature);
}
