#[doc = "Register `INTF_MAGDIFFACC4_LSB` reader"]
pub type R = crate::R<IntfMagdiffacc4LsbSpec>;
#[doc = "Register `INTF_MAGDIFFACC4_LSB` writer"]
pub type W = crate::W<IntfMagdiffacc4LsbSpec>;
#[doc = "Field `INTF_MAGDIFFACC4_LSB` reader - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 3"]
pub type IntfMagdiffacc4LsbR = crate::FieldReader<u32>;
#[doc = "Field `INTF_MAGDIFFACC4_LSB` writer - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 3"]
pub type IntfMagdiffacc4LsbW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 3"]
    #[inline(always)]
    pub fn intf_magdiffacc4_lsb(&self) -> IntfMagdiffacc4LsbR {
        IntfMagdiffacc4LsbR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
This register contains the accumulator value of the interference magnitude difference (LSB 32 bits) for bcnt = 3"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiffacc4_lsb(&mut self) -> IntfMagdiffacc4LsbW<IntfMagdiffacc4LsbSpec> {
        IntfMagdiffacc4LsbW::new(self, 0)
    }
}
#[doc = "INTF_MAGDIFFACC4_LSB\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiffacc4_lsb::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiffacc4_lsb::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffacc4LsbSpec;
impl crate::RegisterSpec for IntfMagdiffacc4LsbSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiffacc4_lsb::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffacc4LsbSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiffacc4_lsb::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffacc4LsbSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFFACC4_LSB to value 0"]
impl crate::Resettable for IntfMagdiffacc4LsbSpec {
    const RESET_VALUE: u32 = 0;
}
