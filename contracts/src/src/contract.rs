#[cfg(not(feature = "library"))]
use cosmwasm_std::entry_point;
use cosmwasm_std::{Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, Order, to_binary, attr};
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, AllPollsResponse, PollResponse, VoteResponse, ConfigResponse};
use cw2::set_contract_version;

use crate::error::ContractError;
use crate::state::{ Config, CONFIG, Poll, POLLS, BALLOTS, Ballot };


const CONTRACT_NAME: &str = "crates.io:cw-starter";
const CONTRACT_VERSION: &str = env!("CARGO_PKG_VERSION");


#[cfg_attr(not(feature = "library"), entry_point)]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: InstantiateMsg,
) -> Result<Response, ContractError> {
    set_contract_version(deps.storage, CONTRACT_NAME, CONTRACT_VERSION)?;
    let admin = msg.admin.unwrap_or(info.sender.to_string());
    let validated_admin = deps.api.addr_validate(&admin)?;
    let config = Config {
        admin: validated_admin.clone(),
    };
    CONFIG.save(deps.storage, &config)?;
    Ok(Response::new()
        .add_attribute("action", "instantiate")
        .add_attribute("admin", validated_admin.to_string()))
}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
    ExecuteMsg::CreatePoll { poll_id, question, options } => execute_create_poll(deps, env, info, poll_id, question, options),
    ExecuteMsg::Vote { poll_id, vote } => execute_vote(deps, env, info, poll_id, vote),
    ExecuteMsg::DeletePool { poll_id } => todo!(),
    ExecuteMsg::DeleteVote { poll_id } => todo!(),
}}

fn execute_create_poll(
    deps: DepsMut,
    _env: Env, 
    info: MessageInfo,
    poll_id: String,
    question: String,
    options: Vec<String>,
) -> Result<Response, ContractError> {
    //Can't have more than 10 options
    if options.len() > 10 {
        return Err(ContractError::TooManyOptions {});
    }

    let mut opts: Vec<(String, u64)> = vec![];
    for option in options.clone() {
        opts.push((option, 0));
    }

    let poll = Poll {
        creator: info.sender,
        question: question.clone(),
        options: opts
    };

    POLLS.save( deps.storage, poll_id.clone(), &poll )?;

    Ok(Response::new().add_attributes(vec![
        attr("poll_id", poll_id),
        attr("question", question),
        attr("num_of_options", options.len().to_string())
    ]))
}

fn execute_vote(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    poll_id: String,
    vote: String,
) -> Result<Response, ContractError> {
    
    match POLLS.load(deps.storage, poll_id.clone()){
        Ok(mut poll) => {
            BALLOTS.update(
                deps.storage,
                (info.sender, poll_id.clone()),
                |ballot| -> StdResult<Ballot> {
                    match ballot {
                        Some(ballot) => {
                            // We need to revoke their old vote
                            // Find the position
                            let position_of_old_vote = poll
                                .options
                                .iter()
                                .position(|option| option.0 == ballot.option)
                                .unwrap();
                            // Decrement by 1
                            poll.options[position_of_old_vote].1 -= 1;
                            // Update the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                        None => {
                            // Simply add the ballot
                            Ok(Ballot { option: vote.clone() })
                        }
                    }
                },
            )?;

            // Find the position of the new vote option and increment it by 1
            let position = poll
                .options
                .iter()
                .position(|option| option.0 == vote);

            if position.is_none() {
                return Err(ContractError::OptionNoExist {  });
            }

            let position = position.unwrap();
            poll.options[position].1 += 1;

            // Save the update
            POLLS.save(deps.storage, poll_id.clone(), &poll)?;
            Ok(Response::new().add_attributes(vec![
                attr("poll_id", poll_id),
                attr("vote", vote)
            ]))
        },
        Err(_) => Err(ContractError::PollNotFound {}),
    }

}

#[cfg_attr(not(feature = "library"), entry_point)]
pub fn query(deps: Deps, env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::AllPolls{} => query_all_polls(deps, env),
        QueryMsg::Poll{poll_id} => query_poll(deps, env, poll_id),
        QueryMsg::Vote{address,poll_id} => query_vote(deps, env, address, poll_id),
        QueryMsg::Config {  } => query_config(deps, env),
        QueryMsg::AllUserVotes { user } => unimplemented!(),
    }
}

fn query_all_polls(deps: Deps, _env: Env) -> StdResult<Binary> {
    let polls = POLLS
        .range(deps.storage, None, None, Order::Ascending)
        .map(|p| Ok(p?.1))
        .collect::<StdResult<Vec<_>>>()?;

    to_binary(&AllPollsResponse { polls })
}

fn query_poll(deps: Deps, _env: Env, poll_id: String) -> StdResult<Binary> {
    let poll = POLLS.may_load(deps.storage, poll_id)?;
    to_binary(&PollResponse { poll })
}

fn query_vote(deps: Deps, _env: Env, address: String, poll_id: String) -> StdResult<Binary> {
    let validated_address = deps.api.addr_validate(&address).unwrap();
    let vote = BALLOTS.may_load(deps.storage, (validated_address, poll_id))?;

    to_binary(&VoteResponse { vote })
}

fn query_config(deps: Deps, _env: Env) -> StdResult<Binary> {
    let config = CONFIG.load(deps.storage)?;
    to_binary(&ConfigResponse { admin: config.admin.to_string() })
}

#[cfg(test)]
mod tests {
    use cosmwasm_std::{attr, from_binary}; // helper to construct an attribute e.g. ("action", "instantiate")
    use cosmwasm_std::testing::{mock_dependencies, mock_env, mock_info}; // mock functions to mock an environment, message info, dependencies
    use crate::contract::{instantiate, execute, query}; // the contract instantiate function
    use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, AllPollsResponse, PollResponse, VoteResponse, ConfigResponse}; // our instantate method

    // Two fake addresses we will use to mock_info
    pub const ADDR1: &str = "addr1";
    pub const ADDR2: &str = "addr2";

    #[test]
    fn test_instantiate() {

        let mut deps = mock_dependencies();
        let env = mock_env();        
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR1)]
        )
    }

    #[test]
    fn test_instantiate_with_admin() {

        let mut deps = mock_dependencies();
        let env = mock_env();        
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: Some( String::from(ADDR2) ) };
        let res = instantiate(deps.as_mut(), env, info, msg).unwrap();

        assert_eq!(
            res.attributes,
            vec![attr("action", "instantiate"), attr("admin", ADDR2)]
        )
    }

    #[test]
    fn test_execute_create_poll_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // New execute msg
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };

        // Unwrap to assert success
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("poll_id","some_id".to_string() ),
                attr("question", "What's your favourite Cosmos coin?".to_string()),
                attr("num_of_options", 3.to_string())
                ]);
    }

    #[test]
    fn test_execute_create_poll_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite number?".to_string(),
            options: vec![
                "1".to_string(),
                "2".to_string(),
                "3".to_string(),
                "4".to_string(),
                "5".to_string(),
                "6".to_string(),
                "7".to_string(),
                "8".to_string(),
                "9".to_string(),
                "10".to_string(),
                "11".to_string(),
            ],
        };

        // Unwrap error to assert failure
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match err{
            crate::ContractError::TooManyOptions {  } => {},
            _ => panic!("{}", err.to_string()),
        }
    }

    #[test]
    fn test_execute_vote_valid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("poll_id","some_id".to_string() ),
                attr("question", "What's your favourite Cosmos coin?".to_string()),
                attr("num_of_options", 3.to_string())
                ]);

        // Create the vote, first time voting
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("poll_id","some_id".to_string() ),
                attr("vote", "Juno".to_string())
                ]);    

        // Change the vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Osmosis".to_string(),
        };
        let res = execute(deps.as_mut(), env, info, msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("poll_id","some_id".to_string() ),
                attr("vote", "Osmosis".to_string())
                ]);    
    }

    #[test]
    fn test_execute_vote_invalid() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create the vote, some_id poll is not created yet.
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "Juno".to_string(),
        };
        // Unwrap to assert error
        let err = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap_err();
        match err{
            crate::ContractError::PollNotFound {  } => {},
            _ => panic!("{}", err.to_string()),
        }

        // Create the poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
        assert_eq!(
            res.attributes,
            vec![
                attr("poll_id","some_id".to_string() ),
                attr("question", "What's your favourite Cosmos coin?".to_string()),
                attr("num_of_options", 3.to_string())
                ]);


        // Vote on a now existing poll but the option "DVPN" does not exist
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id".to_string(),
            vote: "DVPN".to_string(),
        };
        let err = execute(deps.as_mut(), env, info, msg).unwrap_err();
        match err{
            crate::ContractError::OptionNoExist {  } => {},
            _ => panic!("{}", err.to_string()),
        }
    }

    #[test]
    fn test_query_all_polls() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Query before polls are created
        let msg = QueryMsg::AllPolls {};
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: AllPollsResponse = from_binary(&bin).unwrap();
        assert_eq!(res.polls.len(), 0);

        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();

        // Create a second poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_2".to_string(),
            question: "What's your colour?".to_string(),
            options: vec!["Red".to_string(), "Green".to_string(), "Blue".to_string()],
        };
        let _res = execute(deps.as_mut(), env.clone(), info, msg).unwrap();

        // Query
        let msg = QueryMsg::AllPolls {};
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: AllPollsResponse = from_binary(&bin).unwrap();
        assert_eq!(res.polls.len(), 2);
    }


    #[test]
    fn test_query_poll() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        // Query for the poll that exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_1".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect a poll
        assert!(res.poll.is_some());
    
        // Query for the poll that does not exists
        let msg = QueryMsg::Poll {
            poll_id: "some_id_not_exist".to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: PollResponse = from_binary(&bin).unwrap();
        // Expect none
        assert!(res.poll.is_none());
    }

    #[test]
    fn test_query_vote() {
        let mut deps = mock_dependencies();
        let env = mock_env();
        let info = mock_info(ADDR1, &vec![]);
        // Instantiate the contract
        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        // Create a poll
        let msg = ExecuteMsg::CreatePoll {
            poll_id: "some_id_1".to_string(),
            question: "What's your favourite Cosmos coin?".to_string(),
            options: vec![
                "Cosmos Hub".to_string(),
                "Juno".to_string(),
                "Osmosis".to_string(),
            ],
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        // Create a vote
        let msg = ExecuteMsg::Vote {
            poll_id: "some_id_1".to_string(),
            vote: "Juno".to_string(),
        };
        let _res = execute(deps.as_mut(), env.clone(), info.clone(), msg).unwrap();
    
        // Query for a vote that exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_1".to_string(),
            address: ADDR1.to_string(),
        };
        let bin = query(deps.as_ref(), env.clone(), msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to exist
        assert!(res.vote.is_some());
    
        // Query for a vote that does not exists
        let msg = QueryMsg::Vote {
            poll_id: "some_id_2".to_string(),
            address: ADDR2.to_string(),
        };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: VoteResponse = from_binary(&bin).unwrap();
        // Expect the vote to not exist
        assert!(res.vote.is_none());
    }

    #[test]
    fn test_query_config() {

        let mut deps = mock_dependencies();
        let env = mock_env();        
        let info = mock_info(ADDR1, &vec![]);

        let msg = InstantiateMsg { admin: None };
        let _res = instantiate(deps.as_mut(), env.clone(), info, msg).unwrap();

        let msg = QueryMsg::Config {  };
        let bin = query(deps.as_ref(), env, msg).unwrap();
        let res: ConfigResponse = from_binary(&bin).unwrap();
        assert_eq!(res.admin, String::from(ADDR1));

    }
    
}
