use z3_sys::*;
use Context;
use Ast;
use Z3_MUTEX;
use std;

fn mk_interp<'ctx>(ctx: &'ctx Context, ast: &Ast<'ctx>) -> Ast<'ctx> {
    Ast::new(ctx, unsafe {
        let guard = Z3_MUTEX.lock().unwrap();
        Z3_mk_interpolant(ctx.z3_ctx, ast.z3_ast)
    })
}

pub fn compute_interpolant<'ctx>(left: &Ast<'ctx>, right: &Ast<'ctx>) -> Option<Ast<'ctx>> {
    let ctx = left.ctx;
    let left = mk_interp(ctx, left);
    let conj = left.and(&[right]);

    unsafe {
        let params = Z3_mk_params(ctx.z3_ctx);
        let mut interpolants = std::mem::uninitialized();
        let mut model = std::mem::uninitialized();
        let sat = Z3_compute_interpolant(ctx.z3_ctx, conj.z3_ast, params, &mut interpolants,
                                         &mut model);
        if sat == Z3_L_FALSE && Z3_ast_vector_size(ctx.z3_ctx, interpolants) >= 1 {
            return Some(Ast::new(ctx, Z3_ast_vector_get(ctx.z3_ctx, interpolants, 0)));
        } else {
            return None;
        }
    }
}
