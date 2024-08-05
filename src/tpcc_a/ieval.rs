#[doc = "Register `IEVAL` reader"]
pub type R = crate::R<IevalSpec>;
#[doc = "Register `IEVAL` writer"]
pub type W = crate::W<IevalSpec>;
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
#[doc = "Field `RES69` reader - 31:2\\]
RESERVE FIELD"]
pub type Res69R = crate::FieldReader<u32>;
#[doc = "Field `RES69` writer - 31:2\\]
RESERVE FIELD"]
pub type Res69W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
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
    pub fn res69(&self) -> Res69R {
        Res69R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Interrupt Evaluate: CPU write of '1' to the EVALn bit causes the tpcc_intN output signal to be pulsed if any enabled interrupts (IERn) are still pending (IPRn). CPU write of '0' has no effect.."]
    #[inline(always)]
    #[must_use]
    pub fn eval(&mut self) -> EvalW<IevalSpec> {
        EvalW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Interrupt Set: CPU write of '1' to the SETn bit causes the tpcc_intN output signal to be pulsed egardless of state of interrupts enable (IERn) and status (IPRn). CPU write of '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<IevalSpec> {
        SetW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res69(&mut self) -> Res69W<IevalSpec> {
        Res69W::new(self, 2)
    }
}
#[doc = "Interrupt Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`ieval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ieval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IevalSpec;
impl crate::RegisterSpec for IevalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ieval::R`](R) reader structure"]
impl crate::Readable for IevalSpec {}
#[doc = "`write(|w| ..)` method takes [`ieval::W`](W) writer structure"]
impl crate::Writable for IevalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets IEVAL to value 0"]
impl crate::Resettable for IevalSpec {
    const RESET_VALUE: u32 = 0;
}
