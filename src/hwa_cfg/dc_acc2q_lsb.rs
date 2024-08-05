#[doc = "Register `DC_ACC2Q_LSB` reader"]
pub type R = crate::R<DcAcc2qLsbSpec>;
#[doc = "Register `DC_ACC2Q_LSB` writer"]
pub type W = crate::W<DcAcc2qLsbSpec>;
#[doc = "Field `DC_ACC2Q_LSB` reader - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator Q channel for bcnt=1"]
pub type DcAcc2qLsbR = crate::FieldReader<u32>;
#[doc = "Field `DC_ACC2Q_LSB` writer - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator Q channel for bcnt=1"]
pub type DcAcc2qLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator Q channel for bcnt=1"]
    #[inline(always)]
    pub fn dc_acc2q_lsb(&self) -> DcAcc2qLsbR {
        DcAcc2qLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator Q channel for bcnt=1"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc2q_lsb(&mut self) -> DcAcc2qLsbW<DcAcc2qLsbSpec> {
        DcAcc2qLsbW::new(self, 0)
    }
}
#[doc = "DC_ACC2Q_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2q_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2q_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAcc2qLsbSpec;
impl crate::RegisterSpec for DcAcc2qLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc2q_lsb::R`](R) reader structure"]
impl crate::Readable for DcAcc2qLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc2q_lsb::W`](W) writer structure"]
impl crate::Writable for DcAcc2qLsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC2Q_LSB to value 0"]
impl crate::Resettable for DcAcc2qLsbSpec {
    const RESET_VALUE: u32 = 0;
}
