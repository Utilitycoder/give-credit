soroban contract invoke \
    --id $(cat ./.soroban/contract-id) \
    --source givecredit \
    --network testnet \
    -- initialize \
    --admin givecredit \
    --decimal 7 \
    --fee_address GDZJSPRSBTAJPAQ4NG6Y2ZCWHEX5HMS253TYVNAQJRPJHY27JPOHBIPZ \
    --nft_address CAQMKIM64UCAILAERAK56DBFQ36J6MBX4UFDS6UKTTWMRQAN3EIQBWY6 \
