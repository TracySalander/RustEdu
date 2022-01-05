import Array "mo:base/Array";
import Int "mo:base/Int";
import Debug "mo:base/Debug";

actor Main {
    func quicksort(arr: [var Int]){
        let n = arr.size();
        if (n < 2){
            return;
        } else {
            sortHelper(arr, 0, n-1);
        };
    };

    func sortHelper(arr: [var Int], l: Int, r: Int,){
        if (l < r){
            var i = l;
            var j = r;
            var swap = arr[0];
            let pivot = arr[Int.abs(l+r)/2];
            while (i <= j){
                while (arr[Int.abs(i)] < pivot){
                    i += 1;
                };
                while (arr[Int.abs(j)] > pivot){
                    j -= 1;
                };
                if (i <= j){
                    swap := arr[Int.abs(i)];
                    arr[Int.abs(i)] := arr[Int.abs(j)];
                    arr[Int.abs(j)] := swap;
                    i += 1;
                    j -= 1;
                };
            };
            if (l < j){
                sortHelper(arr, l, j);
            };
            if (i < r){
                sortHelper(arr, i, r);
            };
        };
    };

    public func qsort(arr: [Int]): async [Int]{
        let result = Array.thaw<Int>(arr); 
        quicksort(result);
        return Array.freeze<Int>(result);
    };
};
