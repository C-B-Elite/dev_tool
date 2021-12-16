import Cycles "mo:base/ExperimentalCycles";
actor {

    private var balance = 0;

    public shared func inc() : async Nat{
        balance += 1;
        balance  
    };

    public query func get() : async Nat{
        balance
    };

    public query func getBalance() : async Nat{
        Cycles.balance()
    }

}
