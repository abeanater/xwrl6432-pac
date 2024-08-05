#[doc = "Register `IEVAL_RN` reader"]
pub type R = crate::R<IevalRnSpec>;
#[doc = "Register `IEVAL_RN` writer"]
pub type W = crate::W<IevalRnSpec>;
#[doc = "Field `EVAL` reader - 0:0\\]
Interrupt Evaluate: CPU write of '1' to the EVALn bit causes the tpcc_intN output signal to be pulsed if any enabled interrupts (IERn) are still pending (IPRn). CPU write of '0' has no effect.."]
pub type EvalR = crate::BitReader;
#[doc = "Field `EVAL` writer - 0:0\\]
Interrupt Evaluate: CPU write of '1' to the EVALn bit causes the tpcc_intN output signal to be pulsed if any enabled interrupts (IERn) are still pending (IPRn). CPU write of '0' has no effect.."]
pub type EvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET` reader - 1:1\\]
Interrupt Set: CPU write of '1' to the SETn bit causes the tpcc_intN output signal to be pulsed egardless of state of interrupts enable (IERn) and status (IPRn). CPU write of '0' has no effect."]
pub type SetR = crate::BitReader;
#[doc = "Field `SET` writer - 1:1\\]
Interrupt Set: CPU write of '1' to the SETn bit causes the tpcc_intN output signal to be pulsed egardless of state of interrupts enable (IERn) and status (IPRn). CPU write of '0' has no effect."]
pub type SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES76` reader - 31:2\\]
RESERVE FIELD"]
pub type Res76R = crate::FieldReader<u32>;
#[doc = "Field `RES76` writer - 31:2\\]
RESERVE FIELD"]
pub type Res76W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Evaluate: CPU write of '1' to the EVALn bit causes the tpcc_intN output signal to be pulsed if any enabled interrupts (IERn) are still pending (IPRn). CPU write of '0' has no effect.."]
    #[inline(always)]
    pub fn eval(&self) -> EvalR {
        EvalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Set: CPU write of '1' to the SETn bit causes the tpcc_intN output signal to be pulsed egardless of state of interrupts enable (IERn) and status (IPRn). CPU write of '0' has no effect."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res76(&self) -> Res76R {
        Res76R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Evaluate: CPU write of '1' to the EVALn bit causes the tpcc_intN output signal to be pulsed if any enabled interrupts (IERn) are still pending (IPRn). CPU write of '0' has no effect.."]
    #[inline(always)]
    #[must_use]
    pub fn eval(&mut self) -> EvalW<IevalRnSpec> {
        EvalW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Set: CPU write of '1' to the SETn bit causes the tpcc_intN output signal to be pulsed egardless of state of interrupts enable (IERn) and status (IPRn). CPU write of '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<IevalRnSpec> {
        SetW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res76(&mut self) -> Res76W<IevalRnSpec> {
        Res76W::new(self, 2)
    }
}
#[doc = "Interrupt Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ieval_rn::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieval_rn::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IevalRnSpec;
impl crate::RegisterSpec for IevalRnSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieval_rn::R`](R) reader structure"]
impl crate::Readable for IevalRnSpec {}
#[doc = "`write(|w| ..)` method takes [`ieval_rn::W`](W) writer structure"]
impl crate::Writable for IevalRnSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEVAL_RN to value 0"]
impl crate::Resettable for IevalRnSpec {
    const RESET_VALUE: u32 = 0;
}
