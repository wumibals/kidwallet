/// A stub model representing a KidWallet account.
class WalletModel {
  /// The current balance in stroops (1 XLM = 10,000,000 stroops).
  final int balance;

  /// The Stellar public key address for this wallet.
  final String address;

  const WalletModel({
    required this.balance,
    required this.address,
  });
}