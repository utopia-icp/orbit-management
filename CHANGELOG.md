## Unreleased

### Feat

- add notifications for the requester for failed/rejected request (#232)
- show acceptance rules for proposal (#220)
- add metrics dapp label to stations (#229)
- let user add station name (#223)
- show deploy wallet quota exceeded error screen (#222)
- model props use next_time instead of time (#221)
- expose detailed policy evaluation results to users (#213)
- add verified actor option to canister calls (#215)
- record active user sessions (#214)
- harden wallet initialization (#216)
- add orbit station metrics (#212)
- add dynamic metrics service discovery (#211)
- add traits for application metrics (#210)
- add comprehensive set of proposal validation (#199)
- show if raw_rand was successful in system info (#207)
- no upgrader upgrades triggered by wallet upgrade (#204)
- add user status index to control panel (#198)
- add cache to static assets (#197)
- add social share meta fields (#194)
- add split screen + waitlist UI (#190)
- support policy evaluations with indexes (#174)
- get_waiting_list in control panel canister (#186)
- can_deploy_wallet in control panel canister (#168)
- integrate canfund with the control-panel and wallet canisters (#188)
- create canfund lib to monitor and fund canisters (#185)
- reuse metrics setup across canisters (#184)
- add criteria for address in address book (#183)
- accounts use fine grained access control (#182)
- add system_info with version and cycles (#178)
- add control panel endpoint for controllers to update waiting list status (#181)
- add per user locks in deploy_wallet (#171)
- only allow unique user names (#175)
- add content-security-headers (#173)
- add endpoint for requesting user authorization in control panel (#164)
- block anonymous principals (#170)
- set user e-mail in control panel (#163)
- unique time calls in the same round (#166)
- restrict the maximum number of deployed wallets (#165)
- add fields to User struct in control panel to support allow list (#159)
- efficient voting by auto-loading the next proposal (#160)
- add comment when voting; display votes on proposal (#155)
- add description field to wallet (#151)
- add transaction hash to executed ICP transfer (#150)
- Add french (fr) localisation (#144)
- add batch import of transfers (#139)
- add proposal export report (#138)
- added list and detail view for all operations (#137)
- added error pages to router navigation (#136)
- add notifications panel (#132)
- add json view for unknown operations (#131)
- add address book page (#130)
- add account detail page (#129)
- add proposal policies mgmt page (#128)
- add wallet upgrade (#126)
- useStorage composable; persist selected wallet id (#123)
- add permissions mgmt page (#119)
- augment list access policies response (#122)
- add remove wallet dialog (#121)
- check address book metadata when evaluating transfer proposals (#120)
- add wallet dialog (#116)
- add proposal list page (#114)
- address book integration tests (#117)
- add proposals to remove address book entries (#115)
- add proposals to create and update address book entries (#113)
- add recent proposals component (#111)
- expired session overlay (#104)
- get and list methods for address book entries (#110)
- add list proposal sort_by options (#109)
- a new enum for changing metadata (#107)
- add paginated search and more filters to list_proposals (#108)
- add user management page (#102)
- surface user groups in wallet configuration call (#101)
- add user groups management page (#97)
- add authorization layer to ui (#94)
- address book service (#95)
- address book repositories and indices (#93)
- address book entry type (#90)
- rate limit user registration (#84)
- Support custom domains for Orbit UI (#82)
- add proposal type for upgrading an arbitrary canister (#79)
- add more metrics to control-panel (#74)
- add proposal notifications leveraging policy criterias (#75)
- add a len and is_empty to repositories (#81)
- add basic metrics to wallet (#73)
- initial metrics endpoint for control-panel (#69)
- add wallet wasm installation mode (#71)
- add me endpoint to fetch user privileges (#70)
- add operations to mutate proposal policies (#68)
- add operations to mutate access policies (#67)
- add list and get of proposal policies (#65)
- add list and get for access control policies (#64)
- implement user group list and get (#61)
- add control panel option to deploy wallet wasms (#62)
- add list users endpoint (#63)
- add access control with action based resource policies (#60)
- execute upgrades (#59)
- add upgrade proposals (#57)
- add updated user model (#58)
- add additional policy evaluation code (#56)
- policy specifiers (#53)
- add user group proposals (#50)
- add account proposal operations (#44)
- add option to finalize proposal execution async (#39)
- bootstrap integration tests (#36)
- bank self-upgrading mechanism (#19)
- added wallet management user interface (#11)
- added bank canister asset management core (#9)
- add control-panel canister integrated with the wallet ui (#7)
- support login with internet identity (#6)
- add core logger using pino (#3)

### Fix

- safari can open ii window (#226)
- harden cargo clippy on CI by including wasm32-unknown-unknown target (#225)
- increase upgrader canister creation retry period (#217)
- use time in initial rng seed (#206)
- init rng in canister timers (#205)
- do not panic during proposal execution (#191)
- deploy_wallet in control panel should not drop existing wallets (#176)
- canisters with specified IDs must have canister IDs from mainnet ranges (#177)
- principals should be unique per user (#169)
- calculate module hash in the canister (#167)
- skip invalid rows for batch transfer amount sum (#145)
- change the label on the edit wallet save button (#143)
- avoid hitting the instruction limit (#142)
- parse min votes to int (#141)
- do not count inactive user votes (#133)
- pass wallet upgrade params to upgrader and back (#125)
- use user status in auth check (#112)
- orbit development script (#83)
- tighten candid tests (#76)
- drop unused memory ID (#80)
- trigger multiple rounds in transfer test (#43)
- update dfx path of canister specs (#38)
- use Candid value as upgrade arg (#32)
- make frontend work on local replica (#34)
- make upgrader canister build (#33)
- bank canister permissions
- not found page context menu

### Refactor

- unified metric to observe canister method calls (#236)
- drop requester and owner from user specifier (#224)
- orbit terminology (#219)
- handle missing foreign keys consistently (#196)
- code cleanup, variable naming (#127)
- update frontend to match updated api apecs (#92)
- split proposal handler trait (#54)
- make operations consistent with input types (#49)
- proposal creation flow (#28)
- use authorize middleware with all controllers (#27)
- rename bank to wallet canister (#25)
- add controller with middleware macro (#24)
- proposal flow refactor and introduced notification (#18)
- name refactoring for users and assets (#16)
- use unbounded stable structures (#13)
- control-panel canister (#12)
- add error handling for loading receivables

### Perf

- improved access control checks (#161)
- change serializer to use cbor (#154)
- reduce list_proposals instructions (#153)