/**
 * Program IDL in camelCase format in order to be used in JS/TS.
 *
 * Note that this is only a type helper and is not the actual IDL. The original
 * IDL can be found at `target/idl/libreplex_liquidity.json`.
 */
export type LibreplexLiquidity = {
  "address": "LiquGRWGrp8JKspo8zDDu6qpRmX1p6U3PX2USqiE1eg",
  "metadata": {
    "name": "libreplexLiquidity",
    "version": "0.1.0",
    "spec": "0.1.0",
    "description": "Created with Anchor",
    "repository": "https://github.com/Libreplex/libreplex-program-library"
  },
  "instructions": [
    {
      "name": "bootstrapPool",
      "discriminator": [
        153,
        42,
        148,
        41,
        62,
        60,
        181,
        171
      ],
      "accounts": [
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deployment",
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "payerWrappedSolAccount",
          "writable": true
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "writable": true
        },
        {
          "name": "wrappedSolMint",
          "writable": true,
          "address": "So11111111111111111111111111111111111111112"
        },
        {
          "name": "wrappedSolEscrow",
          "writable": true
        },
        {
          "name": "pool",
          "writable": true
        },
        {
          "name": "wrappedSolVault",
          "writable": true
        },
        {
          "name": "fungibleVault",
          "writable": true
        },
        {
          "name": "wrappedSolTokenVault",
          "writable": true
        },
        {
          "name": "fungibleTokenVault",
          "writable": true
        },
        {
          "name": "wrappedSolVaultLpMint",
          "writable": true
        },
        {
          "name": "fungibleVaultLpMint",
          "writable": true
        },
        {
          "name": "wrappedSolVaultLp",
          "writable": true
        },
        {
          "name": "fungibleVaultLp",
          "writable": true
        },
        {
          "name": "lpMint",
          "writable": true
        },
        {
          "name": "payerLpTokenAccount",
          "writable": true
        },
        {
          "name": "systemProgramLpTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "systemProgram"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "lpMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "ammProgram",
          "address": "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"
        },
        {
          "name": "vaultProgram",
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ],
          "address": "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "fungibleTokenFee",
          "writable": true
        },
        {
          "name": "wrappedSolTokenFee",
          "writable": true
        },
        {
          "name": "feeOwner"
        },
        {
          "name": "metadataProgram"
        },
        {
          "name": "lpMintMetadata",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "createLookupTable",
      "discriminator": [
        74,
        26,
        45,
        214,
        23,
        155,
        143,
        153
      ],
      "accounts": [
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deployment",
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "lookupTable",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "wrappedSolVault",
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  118,
                  97,
                  117,
                  108,
                  116
                ]
              },
              {
                "kind": "const",
                "value": [
                  6,
                  155,
                  136,
                  87,
                  254,
                  171,
                  129,
                  132,
                  251,
                  104,
                  127,
                  99,
                  70,
                  24,
                  192,
                  53,
                  218,
                  196,
                  57,
                  220,
                  26,
                  235,
                  59,
                  85,
                  152,
                  160,
                  240,
                  0,
                  0,
                  0,
                  0,
                  1
                ]
              },
              {
                "kind": "const",
                "value": [
                  245,
                  105,
                  223,
                  222,
                  32,
                  35,
                  51,
                  89,
                  141,
                  199,
                  215,
                  75,
                  29,
                  148,
                  184,
                  98,
                  71,
                  121,
                  193,
                  248,
                  47,
                  30,
                  37,
                  166,
                  91,
                  110,
                  78,
                  248,
                  163,
                  190,
                  155,
                  155
                ]
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                15,
                191,
                232,
                132,
                109,
                104,
                92,
                189,
                198,
                44,
                202,
                126,
                4,
                199,
                232,
                246,
                141,
                204,
                49,
                58,
                179,
                18,
                119,
                226,
                224,
                17,
                42,
                46,
                192,
                224,
                82,
                229
              ]
            }
          }
        }
      ],
      "args": [
        {
          "name": "recentSlot",
          "type": "u64"
        }
      ]
    },
    {
      "name": "fixDeploymentType",
      "discriminator": [
        11,
        198,
        80,
        76,
        225,
        76,
        96,
        252
      ],
      "accounts": [
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deployment",
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "payerWrappedSolAccount",
          "writable": true
        },
        {
          "name": "payerFungibleMintTokenAccount",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "fungibleEscrowTokenAccount",
          "writable": true
        },
        {
          "name": "wrappedSolMint",
          "writable": true,
          "address": "So11111111111111111111111111111111111111112"
        },
        {
          "name": "wrappedSolEscrow",
          "writable": true
        },
        {
          "name": "pool",
          "writable": true
        },
        {
          "name": "wrappedSolVault",
          "writable": true
        },
        {
          "name": "fungibleVault",
          "writable": true
        },
        {
          "name": "wrappedSolTokenVault",
          "writable": true
        },
        {
          "name": "fungibleTokenVault",
          "writable": true
        },
        {
          "name": "wrappedSolVaultLpMint",
          "writable": true
        },
        {
          "name": "fungibleVaultLpMint",
          "writable": true
        },
        {
          "name": "wrappedSolVaultLp",
          "writable": true
        },
        {
          "name": "fungibleVaultLp",
          "writable": true
        },
        {
          "name": "lpMint",
          "writable": true
        },
        {
          "name": "payerLpTokenAccount",
          "writable": true
        },
        {
          "name": "systemProgramLpTokenAccount",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "account",
                "path": "systemProgram"
              },
              {
                "kind": "const",
                "value": [
                  6,
                  221,
                  246,
                  225,
                  215,
                  101,
                  161,
                  147,
                  217,
                  203,
                  225,
                  70,
                  206,
                  235,
                  121,
                  172,
                  28,
                  180,
                  133,
                  237,
                  95,
                  91,
                  55,
                  145,
                  58,
                  140,
                  245,
                  133,
                  126,
                  255,
                  0,
                  169
                ]
              },
              {
                "kind": "account",
                "path": "lpMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                140,
                151,
                37,
                143,
                78,
                36,
                137,
                241,
                187,
                61,
                16,
                41,
                20,
                142,
                13,
                131,
                11,
                90,
                19,
                153,
                218,
                255,
                16,
                132,
                4,
                142,
                123,
                216,
                219,
                233,
                248,
                89
              ]
            }
          }
        },
        {
          "name": "ammProgram",
          "address": "Eo7WjKq67rjJQSZxS6z3YkapzY3eMj6Xy8X5EQVn5UaB"
        },
        {
          "name": "vaultProgram",
          "docs": [
            "Vault program. The pool will deposit/withdraw liquidity from the vault."
          ],
          "address": "24Uqj9JCLxUeoC3hGfh5W3s9FM9uCHDS2SG3LYwBpyTi"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "rent",
          "address": "SysvarRent111111111111111111111111111111111"
        },
        {
          "name": "fungibleTokenFee",
          "writable": true
        },
        {
          "name": "wrappedSolTokenFee",
          "writable": true
        },
        {
          "name": "feeOwner"
        },
        {
          "name": "metadataProgram"
        },
        {
          "name": "lpMintMetadata",
          "writable": true
        }
      ],
      "args": []
    },
    {
      "name": "initialise",
      "discriminator": [
        162,
        198,
        118,
        235,
        215,
        247,
        25,
        118
      ],
      "accounts": [
        {
          "name": "authority"
        },
        {
          "name": "treasury"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  105,
                  113,
                  117,
                  105,
                  100,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "arg",
                "path": "input.seed"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialiseInput"
            }
          }
        }
      ]
    },
    {
      "name": "initialiseV2",
      "discriminator": [
        171,
        46,
        16,
        23,
        6,
        77,
        224,
        137
      ],
      "accounts": [
        {
          "name": "authority"
        },
        {
          "name": "treasury"
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidity",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  108,
                  105,
                  113,
                  117,
                  105,
                  100,
                  105,
                  116,
                  121
                ]
              },
              {
                "kind": "arg",
                "path": "input.seed"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "initialiseInputV2"
            }
          }
        }
      ]
    },
    {
      "name": "join",
      "discriminator": [
        206,
        55,
        2,
        106,
        113,
        220,
        17,
        163
      ],
      "accounts": [
        {
          "name": "receiver",
          "writable": true,
          "signer": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "treasury",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "deploymentConfig",
          "writable": true
        },
        {
          "name": "creatorFeeTreasury",
          "writable": true
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "pooledHashlistMarket",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "nonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "pooledNonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        },
        {
          "name": "sysvarInstructions"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "joinInput"
            }
          }
        }
      ]
    },
    {
      "name": "mint",
      "discriminator": [
        51,
        57,
        225,
        47,
        182,
        146,
        137,
        166
      ],
      "accounts": [
        {
          "name": "receiver",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "treasury",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "deploymentConfig",
          "writable": true
        },
        {
          "name": "creatorFeeTreasury",
          "writable": true
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "pooledHashlistMarket",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "nonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "pooledNonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "pooledNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        },
        {
          "name": "sysvarInstructions"
        }
      ],
      "args": []
    },
    {
      "name": "mintSpl",
      "discriminator": [
        60,
        202,
        23,
        216,
        90,
        246,
        222,
        116
      ],
      "accounts": [
        {
          "name": "receiver",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "authority",
          "signer": true
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "deploymentFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deploymentNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "deploymentConfig",
          "writable": true
        },
        {
          "name": "creatorFeeTreasury",
          "writable": true
        },
        {
          "name": "hashlist",
          "writable": true
        },
        {
          "name": "hashlistMarker",
          "writable": true
        },
        {
          "name": "fungibleMint",
          "writable": true
        },
        {
          "name": "liquidityFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "fungibleTokenAccountReceiver",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true,
          "signer": true
        },
        {
          "name": "liquidityNonFungibleTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "fairLaunch",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        },
        {
          "name": "sysvarInstructions"
        }
      ],
      "args": []
    },
    {
      "name": "prepareNativeEscrow",
      "discriminator": [
        96,
        8,
        106,
        167,
        48,
        97,
        184,
        137
      ],
      "accounts": [
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "wrappedSolMint",
          "writable": true,
          "address": "So11111111111111111111111111111111111111112"
        },
        {
          "name": "escrowWrappedSolAccount",
          "writable": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "tokenProgram",
          "address": "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        }
      ],
      "args": []
    },
    {
      "name": "reduceMintCount",
      "discriminator": [
        157,
        237,
        45,
        50,
        51,
        188,
        99,
        211
      ],
      "accounts": [
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "creator",
          "signer": true
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "libreplexFairLaunchProgram",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": {
              "name": "reduceMintCountInputLiquidity"
            }
          }
        }
      ]
    },
    {
      "name": "relinquishCosigner",
      "discriminator": [
        69,
        8,
        121,
        143,
        73,
        21,
        58,
        18
      ],
      "accounts": [
        {
          "name": "liquidity",
          "writable": true
        },
        {
          "name": "deployment",
          "writable": true
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "libreplexFairLaunchProgram",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        }
      ],
      "args": []
    },
    {
      "name": "swapToFungible",
      "discriminator": [
        255,
        175,
        252,
        80,
        237,
        109,
        10,
        98
      ],
      "accounts": [
        {
          "name": "liquidity"
        },
        {
          "name": "deployment",
          "writable": true,
          "relations": [
            "liquidity"
          ]
        },
        {
          "name": "payer",
          "writable": true,
          "signer": true
        },
        {
          "name": "fungibleMint",
          "writable": true,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "hashlistMarker",
          "writable": true,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "value": [
                  104,
                  97,
                  115,
                  104,
                  108,
                  105,
                  115,
                  116,
                  95,
                  109,
                  97,
                  114,
                  107,
                  101,
                  114
                ]
              },
              {
                "kind": "account",
                "path": "deployment"
              },
              {
                "kind": "account",
                "path": "nonFungibleMint"
              }
            ],
            "program": {
              "kind": "const",
              "value": [
                112,
                243,
                237,
                111,
                49,
                210,
                140,
                36,
                186,
                180,
                134,
                167,
                252,
                133,
                18,
                158,
                172,
                39,
                76,
                141,
                75,
                152,
                3,
                163,
                110,
                123,
                220,
                94,
                36,
                84,
                224,
                18
              ]
            }
          }
        },
        {
          "name": "fungibleSourceTokenAccount",
          "writable": true
        },
        {
          "name": "fungibleTargetTokenAccount",
          "writable": true
        },
        {
          "name": "nonFungibleMint",
          "writable": true
        },
        {
          "name": "nonFungibleSourceTokenAccount",
          "writable": true
        },
        {
          "name": "nonFungibleTargetTokenAccount",
          "writable": true
        },
        {
          "name": "tokenProgram22"
        },
        {
          "name": "tokenProgram"
        },
        {
          "name": "associatedTokenProgram",
          "address": "ATokenGPvbdGVxr1b2hvZbsiqW5xWH25efTNsLJA8knL"
        },
        {
          "name": "systemProgram",
          "address": "11111111111111111111111111111111"
        },
        {
          "name": "sysvarInstructions"
        },
        {
          "name": "fairLaunchProgram",
          "address": "8bvPnYE5Pvz2Z9dE6RAqWr1rzLknTndZ9hwvRE6kPDXP"
        }
      ],
      "args": []
    }
  ],
  "accounts": [
    {
      "name": "deployment",
      "discriminator": [
        66,
        90,
        104,
        89,
        183,
        130,
        64,
        178
      ]
    },
    {
      "name": "liquidity",
      "discriminator": [
        54,
        252,
        249,
        226,
        137,
        172,
        121,
        58
      ]
    }
  ],
  "events": [
    {
      "name": "bootstrap",
      "discriminator": [
        133,
        100,
        255,
        233,
        110,
        0,
        227,
        9
      ]
    },
    {
      "name": "liquidityCreate",
      "discriminator": [
        113,
        32,
        180,
        238,
        155,
        90,
        140,
        152
      ]
    },
    {
      "name": "mint",
      "discriminator": [
        63,
        11,
        213,
        134,
        148,
        194,
        24,
        203
      ]
    }
  ],
  "types": [
    {
      "name": "bootstrap",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "liquidity",
            "type": "pubkey"
          }
        ]
      }
    },
    {
      "name": "deployment",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "creator",
            "type": "pubkey"
          },
          {
            "name": "limitPerMint",
            "type": "u64"
          },
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          },
          {
            "name": "numberOfTokensIssued",
            "type": "u64"
          },
          {
            "name": "decimals",
            "type": "u8"
          },
          {
            "name": "useInscriptions",
            "type": "bool"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requireCreatorCosign",
            "type": "bool"
          },
          {
            "name": "migratedFromLegacy",
            "type": "bool"
          },
          {
            "name": "escrowNonFungibleCount",
            "type": "u64"
          },
          {
            "name": "ticker",
            "type": "string"
          },
          {
            "name": "deploymentTemplate",
            "type": "string"
          },
          {
            "name": "mintTemplate",
            "type": "string"
          },
          {
            "name": "fungibleMint",
            "type": "pubkey"
          },
          {
            "name": "offchainUrl",
            "type": "string"
          },
          {
            "name": "disableSwapCosigner",
            "docs": [
              "when cosigner is active this can be toggled to disable swap cosigner",
              "while still requiring cosigner for other actions such as join"
            ],
            "type": "bool"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                199
              ]
            }
          }
        ]
      }
    },
    {
      "name": "initialiseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "pubkey"
          },
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "initialiseInputV2",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "pubkey"
          },
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "requiredDoubleMints",
            "type": "u32"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "pubkey"
            }
          },
          {
            "name": "deploymentType",
            "type": "u8"
          }
        ]
      }
    },
    {
      "name": "joinInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "pooledMultiplierNumerator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "pooledMultiplierDenominator",
            "type": {
              "option": "u16"
            }
          },
          {
            "name": "userMultiplierNumerator",
            "type": "u16"
          },
          {
            "name": "userMultiplierDenominator",
            "type": "u16"
          }
        ]
      }
    },
    {
      "name": "liquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "pubkey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "bootstrapStartTime",
            "type": {
              "option": "i64"
            }
          },
          {
            "name": "bootstrapRequiresSoldOut",
            "type": "bool"
          },
          {
            "name": "poolBootstrapped",
            "type": "bool"
          },
          {
            "name": "creatorBasisPoints",
            "type": "u64"
          },
          {
            "name": "deployment",
            "type": "pubkey"
          },
          {
            "name": "authority",
            "type": "pubkey"
          },
          {
            "name": "treasury",
            "type": "pubkey"
          },
          {
            "name": "lpRatio",
            "type": "u16"
          },
          {
            "name": "totalMints",
            "type": "u64"
          },
          {
            "name": "poolFeeBasisPoints",
            "type": "u64"
          },
          {
            "name": "lookupTableAddress",
            "type": "pubkey"
          },
          {
            "name": "cosignerProgramId",
            "type": "pubkey"
          },
          {
            "name": "deploymentType",
            "type": "u8"
          },
          {
            "name": "requiredDoubleMints",
            "type": {
              "option": "u32"
            }
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                62
              ]
            }
          }
        ]
      }
    },
    {
      "name": "liquidityCreate",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "id",
            "type": "pubkey"
          },
          {
            "name": "liquidity",
            "type": {
              "defined": {
                "name": "liquidity"
              }
            }
          }
        ]
      }
    },
    {
      "name": "mint",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "liquidity",
            "type": "pubkey"
          },
          {
            "name": "totalMints",
            "type": "u64"
          }
        ]
      }
    },
    {
      "name": "reduceMintCountInputLiquidity",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "maxNumberOfTokens",
            "type": "u64"
          }
        ]
      }
    }
  ]
};
