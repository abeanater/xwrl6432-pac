#[doc = "Register `INTF_MAGACC6_LSB` reader"]
pub type R = crate::R<IntfMagacc6LsbSpec>;
#[doc = "Register `INTF_MAGACC6_LSB` writer"]
pub type W = crate::W<IntfMagacc6LsbSpec>;
#[doc = "Field `INTF_MAGACC6_LSB` reader - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 5"]
pub type IntfMagacc6LsbR = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGACC6_LSB` writer - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 5"]
pub type IntfMagacc6LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 5"]
    #[inline(always)]
    pub fn intf_magacc6_lsb(&self) -> IntfMagacc6LsbR {
        IntfMagacc6LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude (LSB 32 bits) for bcnt = 5"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magacc6_lsb(&mut self) -> IntfMagacc6LsbW<IntfMagacc6LsbSpec> {
        IntfMagacc6LsbW::new(self, 0)
    }
}
#[doc = "INTF_MAGACC6_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magacc6_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magacc6_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagacc6LsbSpec;
impl crate::RegisterSpec for IntfMagacc6LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magacc6_lsb::R`](R) reader structure"]
impl crate::Readable for IntfMagacc6LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magacc6_lsb::W`](W) writer structure"]
impl crate::Writable for IntfMagacc6LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGACC6_LSB to value 0"]
impl crate::Resettable for IntfMagacc6LsbSpec {
    const RESET_VALUE: u32 = 0;
}
