#[doc = "Register `INTF_MAGDIFF_SCALE` reader"]
pub type R = crate::R<IntfMagdiffScaleSpec>;
#[doc = "Register `INTF_MAGDIFF_SCALE` writer"]
pub type W = crate::W<IntfMagdiffScaleSpec>;
#[doc = "Field `INTF_MAGDIFF_SCALE` reader - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block.Default 8= scale of 1.0"]
pub type IntfMagdiffScaleR = crate::FieldReader;
#[doc = "Field `INTF_MAGDIFF_SCALE` writer - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block.Default 8= scale of 1.0"]
pub type IntfMagdiffScaleW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `NU1` reader - "]
pub type Nu1R = crate::FieldReader<u32>;
#[doc = "Field `NU1` writer - "]
pub type Nu1W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block.Default 8= scale of 1.0"]
    #[inline(always)]
    pub fn intf_magdiff_scale(&self) -> IntfMagdiffScaleR {
        IntfMagdiffScaleR::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn nu1(&self) -> Nu1R {
        Nu1R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
impl W {
    #[doc = "Bits 0:7 - 7:0\\]
Unsigned scaler (5.3) applied to INTERFSUM_MAGDIFFn from interference statistics block.Default 8= scale of 1.0"]
    #[inline(always)]
    #[must_use]
    pub fn intf_magdiff_scale(&mut self) -> IntfMagdiffScaleW<IntfMagdiffScaleSpec> {
        IntfMagdiffScaleW::new(self, 0)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    #[must_use]
    pub fn nu1(&mut self) -> Nu1W<IntfMagdiffScaleSpec> {
        Nu1W::new(self, 8)
    }
}
#[doc = "INTF_MAGDIFF_SCALE\n\nYou can [`read`](crate::Reg::read) this register and get [`intf_magdiff_scale::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`intf_magdiff_scale::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IntfMagdiffScaleSpec;
impl crate::RegisterSpec for IntfMagdiffScaleSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`intf_magdiff_scale::R`](R) reader structure"]
impl crate::Readable for IntfMagdiffScaleSpec {}
#[doc = "`write(|w| ..)` method takes [`intf_magdiff_scale::W`](W) writer structure"]
impl crate::Writable for IntfMagdiffScaleSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INTF_MAGDIFF_SCALE to value 0"]
impl crate::Resettable for IntfMagdiffScaleSpec {
    const RESET_VALUE: u32 = 0;
}
