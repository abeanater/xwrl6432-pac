#[doc = "Register `ISUM1LSB` reader"]
pub type R = crate::R<Isum1lsbSpec>;
#[doc = "Register `ISUM1LSB` writer"]
pub type W = crate::W<Isum1lsbSpec>;
#[doc = "Field `ISUM1LSB` reader - 31:0\\]
Sum statistics: These registers contain the sum of the I outputs and Q outputs on a per-iteration basis. Only the statistics for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
pub type Isum1lsbR = crate::FieldReader<u32>;
#[doc = "Field `ISUM1LSB` writer - 31:0\\]
Sum statistics: These registers contain the sum of the I outputs and Q outputs on a per-iteration basis. Only the statistics for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
pub type Isum1lsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
Sum statistics: These registers contain the sum of the I outputs and Q outputs on a per-iteration basis. Only the statistics for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
    #[inline(always)]
    pub fn isum1lsb(&self) -> Isum1lsbR {
        Isum1lsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
Sum statistics: These registers contain the sum of the I outputs and Q outputs on a per-iteration basis. Only the statistics for up to four iterations are recorded in these registers. For larger number of iterations, use Statistics output mode (FFT_OUT_MODE in HW_ACC_PARAM register set)."]
    #[inline(always)]
    #[must_use]
    pub fn isum1lsb(&mut self) -> Isum1lsbW<Isum1lsbSpec> {
        Isum1lsbW::new(self, 0)
    }
}
#[doc = "ISUM1LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`isum1lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`isum1lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Isum1lsbSpec;
impl crate::RegisterSpec for Isum1lsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`isum1lsb::R`](R) reader structure"]
impl crate::Readable for Isum1lsbSpec {}
#[doc = "`write(|w| ..)` method takes [`isum1lsb::W`](W) writer structure"]
impl crate::Writable for Isum1lsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ISUM1LSB to value 0"]
impl crate::Resettable for Isum1lsbSpec {
    const RESET_VALUE: u32 = 0;
}
