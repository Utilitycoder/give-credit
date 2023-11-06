soroban contract invoke \
    --id $(cat ./.soroban/contract-id) \
    --source givecredit \
    --network testnet \
    -- initialize \
    --admin givecredit \
    --decimal 7 \
    --fee_address GDZJSPRSBTAJPAQ4NG6Y2ZCWHEX5HMS253TYVNAQJRPJHY27JPOHBIPZ \
    --nft_address $(cat ../nft-contract/.soroban/contract-id) \
