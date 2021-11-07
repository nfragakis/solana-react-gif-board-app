use anchor_lang::prelude::*;

declare_id!("6rkKcKq4NSYBcDRNLhGRwscuY3AJeXqYsU5Q3vCBHdiw");

#[program]
pub mod gifboard {
  use super::*;
  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<AddGif>, gif_link: String) -> ProgramResult {
      // Get a reference to the account and increment total_gifs.
      let base_account = &mut ctx.accounts.base_account;

      let item = ItemStruct {
          gif_link: gif_link.to_string(),
          user_address: *base_account.to_account_info().key,
      };

      base_account.gif_list.push(item);
      base_account.total_gifs += 1;
      Ok(())
  }
}

// Attach certain variables to the StartStuffOff context.
// init will tell Solana to create a new account owned by our current program.
// payer = user tells our program who's paying for the account to be created. In this case, it's the user calling the function.
// space = 9000 which will allocate 9000 bytes of space for our account.
#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

// Specify what data you want in the AddGif Context.
// Getting a handle on the flow of things :)?
#[derive(Accounts)]
pub struct AddGif<'info> {
    #[account(mut)]
    pub base_account: Account<'info, BaseAccount>,
}

#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}