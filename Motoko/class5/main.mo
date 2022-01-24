import Iter "mo:base/Iter";
import List "mo:base/List";
import Principal "mo:base/Principal";
import Time "mo:base/Time";
import Buffer "mo:base/Buffer";
import Array "mo:base/Array";
import Int "mo:base/Int";

actor {
    private type Message = {
        text : Text;
        author : Text;
        time : Time.Time;
    };
    private type FollowInfo = {
        name: Text;
        id : Text;
    };

    private type Microblog = actor{
        follow: shared (Principal) -> async ();
        follows: shared query() -> async [Principal];
        post: shared (Text,Text) -> async ();
        posts: shared query (Time.Time) -> async [Message];
        timeline: shared (Time.Time) -> async [Message];
        get_name: shared query() -> async ?Text;
        set_name: shared (Text) -> ();
    };

    stable var followed : List.List<Principal> = List.nil();
    private stable var auth: ?Text = null;
    public shared func set_name(t: Text) : async () {
        auth := ?t;
    };

    public query func get_name() : async ?Text {
        auth
    };

    public shared func follow(id: Principal) : async () {
        followed := List.push(id, followed);
    };
    
    public shared func follows() : async [FollowInfo] {
        
        var all : List.List<FollowInfo> = List.nil();

        for(id in Iter.fromList(followed)){
            ignore do ? {
                let canister : Microblog = actor(Principal.toText(id));
                let name = (await canister.get_name()) !;
                let tmp : FollowInfo = {name = name;id =Principal.toText(id)};
                all := List.push<FollowInfo>(tmp, all);

            }
        };
        return List.toArray(all);
    };

    stable var messages : List.List<Message> = List.nil();

    public shared ({caller}) func post(opt: Text,text : Text) : async () {
        assert(opt == "suckmydick");
        let _author = switch (auth) {
            case (?a) { a };
            case (null) { "" };
        };
        var msg : Message = {
            text = text;
            author = _author; 
            time = Time.now();
        };
        messages := List.push(msg, messages);
    };

     public shared query func posts(since : Time.Time) : async [Message] {
        var since_message : List.List<Message> = List.nil();
        for (msg in Iter.fromList(messages)) {
            if (msg.time >= since) {
                since_message := List.push(msg, since_message);
            };
        };
        return List.toArray(since_message);
    };

    public shared func timeline(since : Time.Time) : async [Message] {
        var all : List.List<Message> = List.nil();

        for(id in Iter.fromList(followed)){
            let canister : Microblog = actor(Principal.toText(id));
            let msgs = await canister.posts(since);
            for(msg in Iter.fromArray(msgs)){
                all := List.push(msg, all);
            };
        };

        return List.toArray(all);
    };
};
