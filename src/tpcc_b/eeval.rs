#[doc = "Register `EEVAL` reader"]
pub type R = crate::R<EevalSpec>;
#[doc = "Register `EEVAL` writer"]
pub type W = crate::W<EevalSpec>;
#[doc = "Field `EVAL` reader - 0:0\\]
Error Interrupt Evaluate: CPU write of '1' to the EVAL bit causes the TPCC error interrupt to be pulsed if any errors have not been cleared in the EMR/EMRH QEMR or CCERR registers. CPU write of '0' has no effect."]
pub type EvalR = crate::BitReader;
#[doc = "Field `EVAL` writer - 0:0\\]
Error Interrupt Evaluate: CPU write of '1' to the EVAL bit causes the TPCC error interrupt to be pulsed if any errors have not been cleared in the EMR/EMRH QEMR or CCERR registers. CPU write of '0' has no effect."]
pub type EvalW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SET` reader - 1:1\\]
Error Interrupt Set: CPU write of '1' to the SET bit causes the TPCC error interrupt to be pulsed regardless of state of EMR/EMRH QEMR or CCERR. CPU write of '0' has no effect."]
pub type SetR = crate::BitReader;
#[doc = "Field `SET` writer - 1:1\\]
Error Interrupt Set: CPU write of '1' to the SET bit causes the TPCC error interrupt to be pulsed regardless of state of EMR/EMRH QEMR or CCERR. CPU write of '0' has no effect."]
pub type SetW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RES37` reader - 31:2\\]
RESERVE FIELD"]
pub type Res37R = crate::FieldReader<u32>;
#[doc = "Field `RES37` writer - 31:2\\]
RESERVE FIELD"]
pub type Res37W<'a, REG> = crate::FieldWriter<'a, REG, 30, u32>;
impl R {
    #[doc = "Bit 0 - 0:0\\]
Error Interrupt Evaluate: CPU write of '1' to the EVAL bit causes the TPCC error interrupt to be pulsed if any errors have not been cleared in the EMR/EMRH QEMR or CCERR registers. CPU write of '0' has no effect."]
    #[inline(always)]
    pub fn eval(&self) -> EvalR {
        EvalR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error Interrupt Set: CPU write of '1' to the SET bit causes the TPCC error interrupt to be pulsed regardless of state of EMR/EMRH QEMR or CCERR. CPU write of '0' has no effect."]
    #[inline(always)]
    pub fn set_(&self) -> SetR {
        SetR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    pub fn res37(&self) -> Res37R {
        Res37R::new((self.bits >> 2) & 0x3fff_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - 0:0\\]
Error Interrupt Evaluate: CPU write of '1' to the EVAL bit causes the TPCC error interrupt to be pulsed if any errors have not been cleared in the EMR/EMRH QEMR or CCERR registers. CPU write of '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn eval(&mut self) -> EvalW<EevalSpec> {
        EvalW::new(self, 0)
    }
    #[doc = "Bit 1 - 1:1\\]
Error Interrupt Set: CPU write of '1' to the SET bit causes the TPCC error interrupt to be pulsed regardless of state of EMR/EMRH QEMR or CCERR. CPU write of '0' has no effect."]
    #[inline(always)]
    #[must_use]
    pub fn set_(&mut self) -> SetW<EevalSpec> {
        SetW::new(self, 1)
    }
    #[doc = "Bits 2:31 - 31:2\\]
RESERVE FIELD"]
    #[inline(always)]
    #[must_use]
    pub fn res37(&mut self) -> Res37W<EevalSpec> {
        Res37W::new(self, 2)
    }
}
#[doc = "Error Eval Register\n\nYou can [`read`](crate::Reg::read) this register and get [`eeval::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`eeval::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EevalSpec;
impl crate::RegisterSpec for EevalSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eeval::R`](R) reader structure"]
impl crate::Readable for EevalSpec {}
#[doc = "`write(|w| ..)` method takes [`eeval::W`](W) writer structure"]
impl crate::Writable for EevalSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets EEVAL to value 0"]
impl crate::Resettable for EevalSpec {
    const RESET_VALUE: u32 = 0;
}
