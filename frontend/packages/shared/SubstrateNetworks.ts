import { getEnvironment } from "shared/endpoints";
import { TokenId } from "tokens";

export const SubstrateNetworks = ["kusama", "picasso", "karura", "statemine"] as const;
export type SubstrateNetworkId = typeof SubstrateNetworks[number];
export type SubstrateNetwork = {
  relayChain: "polkadot" | "kusama";
  parachainId: number | 0;
  name: string;
  wsUrl: string;
  tokenId: TokenId;
  ss58Format: number;
  subscanUrl: string;
  decimals: number;
  color?: string;
  symbol: string;
  logo: string;
};


export const SUBSTRATE_NETWORKS: {
  [substrateNetworkId in SubstrateNetworkId]: SubstrateNetwork;
} = {
  kusama: {
    relayChain: "kusama",
    parachainId: 0,
    name: "Kusama",
    wsUrl: getEnvironment("kusama"),
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "https://kusama.subscan.io/",
    decimals: 12,
    symbol: "KSM",
    logo: "/networks/kusama.svg",
  },
  picasso: {
    relayChain: "kusama",
    parachainId: 2087,
    name: "Picasso",
    wsUrl: getEnvironment("picasso"),
    tokenId: "pica",
    ss58Format: 49,
    subscanUrl: "https://picasso.subscan.io/",
    decimals: 12,
    symbol: "PICA",
    logo: "/networks/picasso.svg",
  },
  karura: {
    relayChain: "kusama",
    parachainId: 2000,
    name: "Karura",
    wsUrl: getEnvironment("karura"),
    tokenId: "kar",
    ss58Format: 8,
    subscanUrl: "https://karura.subscan.io/",
    decimals: 12,
    symbol: "KAR",
    logo: "/networks/karura.svg",
  },
  statemine: {
    relayChain: "kusama",
    parachainId: 1000,
    name: "Statemine",
    wsUrl: getEnvironment("statemine"),
    tokenId: "ksm",
    ss58Format: 2,
    subscanUrl: "",
    symbol: "KSM",
    logo: "/networks/statemine.svg",
    decimals: 12,
  },
};

export const getSubstrateNetwork = (
  networkId: SubstrateNetworkId
): SubstrateNetwork => SUBSTRATE_NETWORKS[networkId];

/**
 * TODO: move to utils or substrate-react
 * @param network picasso, karura or kusama
 * @param extrinsicHash extrinsic hash generated by polkadot js
 * @returns link to subscan
 */
export function subscanExtrinsicLink(
  network: SubstrateNetworkId,
  extrinsicHash: string
): string {
  return SUBSTRATE_NETWORKS[network].subscanUrl + "extrinsic/" + extrinsicHash;
}