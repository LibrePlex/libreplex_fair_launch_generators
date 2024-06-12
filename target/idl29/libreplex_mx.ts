export type LibreplexMx = {
  "version": "0.0.0",
  "name": "libreplex_mx",
  "instructions": [
    {
      "name": "join",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metaplexJoiner",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metaplexJoiner",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metaplex_joiner"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.seed"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialiseInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "metaplexJoiner",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "filterValue",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "docs": [
              "0 - collection",
              "1 - creator"
            ],
            "type": "u8"
          },
          {
            "name": "cosigner",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                100
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "cosigner",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "filterValue",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "MetaplexJoinerCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "metaplexJoiner",
          "type": {
            "defined": "MetaplexJoiner"
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NoCreatorMatchingFilter",
      "msg": "No creator matching filter"
    },
    {
      "code": 6001,
      "name": "CreatorNotVerified",
      "msg": "Creator not verified"
    },
    {
      "code": 6002,
      "name": "MetadataCollectionNotVerified",
      "msg": "Collection not verified"
    },
    {
      "code": 6003,
      "name": "MetadataHasInvalidCollection",
      "msg": "Collection not verified"
    }
  ]
};

export const IDL: LibreplexMx = {
  "version": "0.0.0",
  "name": "libreplex_mx",
  "instructions": [
    {
      "name": "join",
      "accounts": [
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metaplexJoiner",
          "isMut": true,
          "isSigner": false,
          "relations": [
            "deployment"
          ]
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "deploymentConfig",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "creatorFeeTreasury",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlist",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMint",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "nonFungibleMetadata",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "hashlistMarker",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": []
    },
    {
      "name": "initialise",
      "accounts": [
        {
          "name": "deployment",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "payer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "creator",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "metaplexJoiner",
          "isMut": true,
          "isSigner": false,
          "pda": {
            "seeds": [
              {
                "kind": "const",
                "type": "string",
                "value": "metaplex_joiner"
              },
              {
                "kind": "arg",
                "type": {
                  "defined": "InitialiseInput"
                },
                "path": "input.seed"
              }
            ]
          }
        },
        {
          "name": "systemProgram",
          "isMut": false,
          "isSigner": false
        },
        {
          "name": "fairLaunch",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "input",
          "type": {
            "defined": "InitialiseInput"
          }
        }
      ]
    }
  ],
  "accounts": [
    {
      "name": "metaplexJoiner",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "bump",
            "type": "u8"
          },
          {
            "name": "creator",
            "type": "publicKey"
          },
          {
            "name": "deployment",
            "type": "publicKey"
          },
          {
            "name": "filterValue",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "docs": [
              "0 - collection",
              "1 - creator"
            ],
            "type": "u8"
          },
          {
            "name": "cosigner",
            "type": "publicKey"
          },
          {
            "name": "cosignerProgramId",
            "type": "publicKey"
          },
          {
            "name": "padding",
            "type": {
              "array": [
                "u8",
                100
              ]
            }
          }
        ]
      }
    }
  ],
  "types": [
    {
      "name": "InitialiseInput",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "seed",
            "type": "publicKey"
          },
          {
            "name": "cosigner",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "cosignerProgramId",
            "type": {
              "option": "publicKey"
            }
          },
          {
            "name": "filterValue",
            "type": "publicKey"
          },
          {
            "name": "filterType",
            "type": "u8"
          }
        ]
      }
    }
  ],
  "events": [
    {
      "name": "MetaplexJoinerCreate",
      "fields": [
        {
          "name": "id",
          "type": "publicKey",
          "index": false
        },
        {
          "name": "metaplexJoiner",
          "type": {
            "defined": "MetaplexJoiner"
          },
          "index": false
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "NoCreatorMatchingFilter",
      "msg": "No creator matching filter"
    },
    {
      "code": 6001,
      "name": "CreatorNotVerified",
      "msg": "Creator not verified"
    },
    {
      "code": 6002,
      "name": "MetadataCollectionNotVerified",
      "msg": "Collection not verified"
    },
    {
      "code": 6003,
      "name": "MetadataHasInvalidCollection",
      "msg": "Collection not verified"
    }
  ]
};
