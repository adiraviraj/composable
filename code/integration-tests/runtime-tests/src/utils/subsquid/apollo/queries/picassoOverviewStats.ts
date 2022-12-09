import gql from "../gql";

export type OverviewStats = {
  overviewStats: {
    accountHoldersCount: number;
    activeUsersCount: number;
    totalValueLocked: string;
    transactionsCount: number;
  };
};

export const OVERVIEW_STATS = gql`
  query overviewStats {
    overviewStats {
      accountHoldersCount
      activeUsersCount
      totalValueLocked
      transactionsCount
    }
  }
`;
