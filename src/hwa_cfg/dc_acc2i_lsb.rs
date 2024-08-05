#[doc = "Register `DC_ACC2I_LSB` reader"]
pub type R = crate::R<DcAcc2iLsbSpec>;
#[doc = "Register `DC_ACC2I_LSB` writer"]
pub type W = crate::W<DcAcc2iLsbSpec>;
#[doc = "Field `DC_ACC2I_LSB` reader - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=1"]
pub type DcAcc2iLsbR = crate::FieldReader<u32>;
#[doc = "Field `DC_ACC2I_LSB` writer - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=1"]
pub type DcAcc2iLsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=1"]
    #[inline(always)]
    pub fn dc_acc2i_lsb(&self) -> DcAcc2iLsbR {
        DcAcc2iLsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register provides the LSB 32 bits value of DC accumulator I channel for bcnt=1"]
    #[inline(always)]
    #[must_use]
    pub fn dc_acc2i_lsb(&mut self) -> DcAcc2iLsbW<DcAcc2iLsbSpec> {
        DcAcc2iLsbW::new(self, 0)
    }
}
#[doc = "DC_ACC2I_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`dc_acc2i_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dc_acc2i_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcAcc2iLsbSpec;
impl crate::RegisterSpec for DcAcc2iLsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dc_acc2i_lsb::R`](R) reader structure"]
impl crate::Readable for DcAcc2iLsbSpec {}
#[doc = "`write(|w| ..)` method takes [`dc_acc2i_lsb::W`](W) writer structure"]
impl crate::Writable for DcAcc2iLsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets DC_ACC2I_LSB to value 0"]
impl crate::Resettable for DcAcc2iLsbSpec {
    const RESET_VALUE: u32 = 0;
}
