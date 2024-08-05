#[doc = "Register `INTF_MAGACC5_LSB` reader"]
pub type R = crate::R<IntfMagacc5LsbSpec>;
#[doc = "Register `INTF_MAGACC5_LSB` writer"]
pub type W = crate::W<IntfMagacc5LsbSpec>;
#[doc = "Field `INTF_MAGACC5_LSB` reader - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 4"]
pub type IntfMagacc5LsbR = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGACC5_LSB` writer - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 4"]
pub type IntfMagacc5LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 4"]
    #[inline(always)]
    pub fn intf_magacc5_lsb(&self) -> IntfMagacc5LsbR {
        IntfMagacc5LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 4"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magacc5_lsb(&mut self) -> IntfMagacc5LsbW<IntfMagacc5LsbSpec> {
        IntfMagacc5LsbW::new(self, 0)
    }
}
#[doc = "INTF_MAGACC5_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc5_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc5_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc5LsbSpec;
impl crate::RegisterSpec for IntfMagacc5LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc5_lsb::R`](R) reader structure"]
impl crate::Readable for IntfMagacc5LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc5_lsb::W`](W) writer structure"]
impl crate::Writable for IntfMagacc5LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC5_LSB to value 0"]
impl crate::Resettable for IntfMagacc5LsbSpec {
    const RESET_VALUE: u32 = 0;
}
