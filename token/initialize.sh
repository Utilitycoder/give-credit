soroban contract invoke \
    --id CBYVJRPUUU6LSALKKLP64YP4LN6PB3P4434TVGVHLUP55S53N66PXQNG \
    --source givecredit \
    --network testnet \
    -- initialize \
    --admin givecredit \
    --decimal 7 \
    --address GDZJSPRSBTAJPAQ4NG6Y2ZCWHEX5HMS253TYVNAQJRPJHY27JPOHBIPZ \

soroban contract invoke \
    --id CBYVJRPUUU6LSALKKLP64YP4LN6PB3P4434TVGVHLUP55S53N66PXQNG \
    --source givecredit \
    --network testnet \
    -- transfer \
    --from GDUIYMXGPHNMMUSUGMMQLF2GKS76WZXWMKZVVORXVR7HLPKIAGYGODFX \
    --to GCRK2BBUCYTZHMNXQJ66ZDRHKREIR3TJ6FVSVMJV2CDI56NXIJUULMPG \
    --address 250 \