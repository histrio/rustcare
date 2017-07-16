extern crate libc;

use std::ptr;
use std::io::Error;

type Address = u64;
type Word = u64;

const PTRACE_GETREGS:libc::c_uint = 12;

#[derive(Clone, Default, Debug)]
struct Registers {
  pub r15: Word,
  pub r14: Word,
  pub r13: Word,
  pub r12: Word,
  pub rbp: Word,
  pub rbx: Word,
  pub r11: Word,
  pub r10: Word,
  pub r9: Word,
  pub r8: Word,
  pub rax: Word,
  pub rcx: Word,
  pub rdx: Word,
  pub rsi: Word,
  pub rdi: Word,
  pub orig_rax: Word,
  pub rip: Word,
  pub cs: Word,
  pub eflags: Word,
  pub rsp: Word,
  pub ss: Word,
  pub fs_base: Word,
  pub gs_base: Word,
  pub ds: Word,
  pub es: Word,
  pub fs: Word,
  pub gs: Word
}

unsafe fn raw(request: libc::c_uint, pid: libc::pid_t, addr: *mut libc::c_void, data: *mut libc::c_void) -> Result<libc::c_long, usize> {
  let v = libc::ptrace(request, pid, addr, data);
  match v {
      -1 => {
          println!("OS error: {:?}", Error::last_os_error());
            Result::Err(0)
        },
      _ => Result::Ok(v)
  }
}

fn getregs(pid: libc::pid_t) -> Result<Registers, usize> {
    let mut buf: Registers = Default::default();
    let buf_mut: *mut Registers = &mut buf;
    match unsafe {
        raw (PTRACE_GETREGS, pid, ptr::null_mut(), buf_mut as *mut libc::c_void)
    } {
        Ok(_) => Ok(buf),
        Err(e) => Err(e)
    }
}

fn peek_data(pid: libc::pid_t, address:Address) -> Result<libc::c_long, usize> {
    unsafe { raw(libc::PTRACE_PEEKDATA, pid, address as *mut libc::c_void, ptr::null_mut()) }
}

fn attach(pid: libc::pid_t) -> Result<libc::c_long, usize> {
    unsafe { raw (libc::PTRACE_ATTACH, pid, ptr::null_mut(), ptr::null_mut()) }
}

fn detach(pid: libc::pid_t) -> Result<libc::c_long, usize>{
    unsafe { raw (libc::PTRACE_DETACH, pid, ptr::null_mut() as *mut libc::c_void, ptr::null_mut() as *mut libc::c_void) }
}

fn wait(pid: libc::pid_t){
    let mut status: i32 = 0;
    unsafe { libc::waitpid(pid, &mut status as *mut libc::c_int, 0) };
}

pub fn patch(pid: libc::pid_t, patch:&str){
    println!("{:?}", patch);
    match attach(pid) {
        Err(f) => println!("Cannot attach to {:?}:{:?}", pid, f),
        Ok(_) => {
            wait(pid);
            println!("Attached to {:?}", pid);
            let regs = getregs(pid).unwrap();
            match peek_data(pid, regs.ds as Address) {
                Ok(m) => println!("{:?}", m),
                Err(f) => println!("Data peek error {:?}", f)
            };
            match detach(pid) {
                Ok(_) => {
                    wait(pid);
                    println!("Detached from {:?}", pid);
                },
                Err(f) => println!("Cannot detach from {:?}:{:?}", pid, f)
            }
        }
    }
}
