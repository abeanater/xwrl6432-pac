#[doc = "Register `RESERVED` reader"]
pub type R = crate::R<ReservedSpec>;
#[doc = "Register `RESERVED` writer"]
pub type W = crate::W<ReservedSpec>;
#[doc = "Field `gio_config` reader - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
pub type GioConfigR = crate::FieldReader<u32>;
#[doc = "Field `gio_config` writer - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
pub type GioConfigW<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
    #[inline(always)]
    pub fn gio_config(&self) -> GioConfigR {
        GioConfigR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:31 - 31:0\\]
bit0 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT0 to IRQ bit1 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT1to IRQ bit2: writing '1' will slect negedge for pulse generation of GIO_PAD_INT2 to IRQ bit3 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT3 to IRQ bit4 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT4 to IRQ bit5 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT5 to IRQ bit6 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT6 to IRQ bit7 : writing '1' will slect negedge for pulse generation of GIO_PAD_INT7 to IRQ"]
    #[inline(always)]
    #[must_use]
    pub fn gio_config(&mut self) -> GioConfigW<ReservedSpec> {
        GioConfigW::new(self, 0)
    }
}
#[doc = "RESERVED\n\nYou can [`read`](crate::Reg::read) this register and get [`reserved::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reserved::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ReservedSpec;
impl crate::RegisterSpec for ReservedSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reserved::R`](R) reader structure"]
impl crate::Readable for ReservedSpec {}
#[doc = "`write(|w| ..)` method takes [`reserved::W`](W) writer structure"]
impl crate::Writable for ReservedSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RESERVED to value 0"]
impl crate::Resettable for ReservedSpec {
    const RESET_VALUE: u32 = 0;
}
