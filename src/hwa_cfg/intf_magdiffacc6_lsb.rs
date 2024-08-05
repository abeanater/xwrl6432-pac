#[doc = "Register `INTF_MAGDIFFACC6_LSB` reader"]
pub type R = crate::R<IntfMagdiffacc6LsbSpec>;
#[doc = "Register `INTF_MAGDIFFACC6_LSB` writer"]
pub type W = crate::W<IntfMagdiffacc6LsbSpec>;
#[doc = "Field `INTF_MAGDIFFACC6_LSB` reader - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 5"]
pub type IntfMagdiffacc6LsbR = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGDIFFACC6_LSB` writer - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 5"]
pub type IntfMagdiffacc6LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 5"]
    #[inline(always)]
    pub fn intf_magdiffacc6_lsb(&self) -> IntfMagdiffacc6LsbR {
        IntfMagdiffacc6LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 5"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiffacc6_lsb(&mut self) -> IntfMagdiffacc6LsbW<IntfMagdiffacc6LsbSpec> {
        IntfMagdiffacc6LsbW::new(self, 0)
    }
}
#[doc = "INTF_MAGDIFFACC6_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc6_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc6_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffacc6LsbSpec;
impl crate::RegisterSpec for IntfMagdiffacc6LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiffacc6_lsb::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffacc6LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiffacc6_lsb::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffacc6LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFFACC6_LSB to value 0"]
impl crate::Resettable for IntfMagdiffacc6LsbSpec {
    const RESET_VALUE: u32 = 0;
}
