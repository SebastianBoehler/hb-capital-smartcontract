use anchor_lang::prelude::*;

declare_id!("2gjJ2mxUjGx1hsnBv6TnpTuZJ8B2nq5JYbMenEC2YijZ");

#[account]
pub struct Order {
    pub time: i64,
    // type 0: buy, 1: sell
    pub order_type: u8,
    // side 0: long, 1: short
    pub side: u8,
    pub price: u64,
}

#[account]
pub struct Action {
    //type 0: margin change, 1: lever change
    pub action_type: u8,
    pub time: i64,
    pub set_to: u64,
}

#[account]
pub struct Position {
    pub ticker: String,
    // array of actions
    pub actions: Vec<Action>,
    // array of orders
    pub orders: Vec<Order>,
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct Initialize<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    // Setup Position PDA
    #[account(
        init, payer = signer, space = 8 + 4 + 6 + (1 + 8 + 8) * 100 + (8 + 1 + 1 + 8) * 100, // 100 orders and actions
        seeds = [b"pos", ticker.as_bytes(), signer.key().as_ref()], bump
    )]
    pub position: Account<'info, Position>,
    pub system_program: Program<'info, System>,
}

#[derive(Accounts)]
#[instruction(ticker: String)]
pub struct AddOrder<'info> {
    #[account(mut)]
    pub signer: Signer<'info>,

    //Position PDA
    #[account(
        mut,
        seeds = [b"pos", ticker.as_bytes(), signer.key().as_ref()], bump
    )]
    pub position: Account<'info, Position>,
}

#[program]
pub mod hb_capital_smartcontract {
    use super::*;

    pub fn initialize(ctx: Context<Initialize>, ticker: String, _bump: u8) -> Result<()> {
        ctx.accounts.position.ticker = ticker;
        ctx.accounts.position.actions = Vec::new();
        ctx.accounts.position.orders = Vec::new();
        msg!("Initialized");
        Ok(())
    }

    pub fn add_order(ctx: Context<AddOrder>, _ticker: String, order_type: u8, side: u8, price: u64, time: i64) -> Result<()> {
        let order = Order {
            time,
            order_type,
            side,
            price,
        };
        ctx.accounts.position.orders.push(order);
        //msg with params
        msg!("Added order: type:{} side:{} price:{} time:{}", order_type, side, price, time);
        Ok(())
    }
}

#[error_code]
pub enum ErrorCode {
    #[msg("Unauthorized")]
    Unathorized,
}
